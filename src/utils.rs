use super::constants::*;

use jsonrpsee::core::Serialize;
use reth::providers::StateProvider;
use reth::revm::database::StateProviderDatabase;
use reth::revm::db::{CacheDB, DatabaseRef};
use reth::revm::EVM;
use reth::primitives::{Address, U256, Bytes, keccak256};
use alloy_dyn_abi::DynSolValue;
use alloy_sol_types::{SolCall, sol};
use reth::revm::primitives::{TransactTo, ResultAndState, ExecutionResult};
use serde::Deserialize;

type StateProviderBox = Box<dyn StateProvider>;
type StateProviderDB = StateProviderDatabase<StateProviderBox>;
type CacheDBStateProvider = CacheDB<StateProviderDB>;
type EvmStateProvider = EVM<CacheDBStateProvider>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaxCallResult {
    Success(TaxInfo),
    CallError(TaxCallError)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxInfo {
    pub token: Address,
    pub buy: Option<U256>,
    pub sell: Option<U256>,
}

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum TaxCallError {
    #[error("No pair address found for token {0:?}")]
    PairAddressDoesNotExist(Address),
    #[error("Failed to fetch pair address for token {0:}: {1:?}")]
    CallingPairReverts(Address, String),
    #[error("Get pair call halt")]
    CallingPairHalt,
    #[error("Failed to fetch token tax {0:}: {1:?}")]
    CallingTaxError(Address, String),
}

impl TaxInfo {
    pub fn new(token: Address) -> Self {
        Self {
            token,
            buy: None,
            sell: None,
        }
    }

    // atm pair address can only be found from the uniwapv2 factory contract
    pub fn get_pair_address(&self, evm: &mut EvmStateProvider) -> Result<Address, TaxCallError> 
    {
        sol! {
            function getPair(address token0, address token1) returns (address pair);
        }

        let weth: Address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse().unwrap();
        let univ2_factory: Address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse().unwrap();
        let (token_0, token_1) = if weth < self.token {
            (weth, self.token)
        } else {
            (self.token, weth)
        };

        let call = getPairCall {
            token0: token_0,
            token1: token_1,
        };
        let tx_data = call.abi_encode();
        
        evm.env.tx.data = tx_data.into();
        evm.env.tx.transact_to = TransactTo::Call(univ2_factory);
        let ResultAndState { result, .. } = evm.transact().unwrap();

        match result {
            ExecutionResult::Success { output, .. } => {
                let decoded = getPairCall::abi_decode_returns(output.data(), true).unwrap();
                let pair_address = decoded.pair;
                if pair_address != Address::ZERO {
                    return Ok(pair_address);
                } else {
                    return Err(TaxCallError::PairAddressDoesNotExist(self.token).into())
                }
            },
            ExecutionResult::Revert { gas_used: _, output } => {
                return Err(TaxCallError::CallingPairReverts(self.token, output.to_string()))
            },
            ExecutionResult::Halt { .. } => {
                return Err(TaxCallError::CallingPairHalt)
            }
        }
    }

    pub fn encode_call(&self, pair: Address) -> Vec<u8> {
        let call = getTaxCall {
            pair,
            tokenIn: self.token,
            dexFee: U256::from(997),
        };
        call.abi_encode()
    }
    
    pub fn decode_success_call(&mut self, output: Bytes) {
        let decoded = getTaxCall::abi_decode_returns(&output, true).unwrap();
        let buy_tax = decoded.buyTax;
        let sell_tax = decoded.sellTax;
        if buy_tax > U256::from(99) {
            self.buy = Some(buy_tax / U256::from(100))
        }
        if sell_tax > U256::from(99) {
            self.sell = Some(sell_tax / U256::from(100))
        }
    }
}



pub fn insert_fake_approval<ExtDB>(token: Address, pair: Address, db: &mut CacheDB<ExtDB>)
where
    ExtDB: DatabaseRef,
    <ExtDB as DatabaseRef>::Error: std::fmt::Debug,
{
    for i in 0..100 {
        let slot_new = map_location(U256::from(i), pair, tax_checker_address());
        let max_uint = U256::MAX;
        db.insert_account_storage(token, slot_new, max_uint).unwrap();
    }
}

pub fn map_location(slot: U256, key: Address, key_after: Address) -> U256 {
    let input = [DynSolValue::Address(key), DynSolValue::Uint(slot, 32)];
    let input = DynSolValue::Tuple(input.to_vec());
    let key_slot_hash: U256 = keccak256(input.abi_encode()).into();

    let input = [DynSolValue::Address(key_after), DynSolValue::Uint(key_slot_hash, 32)];
    let input = DynSolValue::Tuple(input.to_vec());
    let slot: U256 = keccak256(input.abi_encode()).into();
    slot
}

sol! {
    #[derive(Debug)]
    function getTax(
        address pair, 
        address tokenIn, 
        uint256 dexFee
    ) returns (uint256 buyTax, uint256 sellTax);
}