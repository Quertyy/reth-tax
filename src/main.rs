mod constants;
mod utils;
use utils::*;
use constants::*;

use clap::Parser;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, types::ErrorObject};
use reth::cli::{
    components::{RethNodeComponents, RethRpcComponents},
    config::RethRpcConfig,
    ext::{RethCliExt, RethNodeCommandConfig},
    Cli,
};
use reth::rpc::eth::EthTransactions;
use reth::primitives::{BlockId, BlockNumberOrTag, keccak256, KECCAK_EMPTY};
use reth::revm::database::StateProviderDatabase;
use reth::revm::primitives::{Address, Env, ResultAndState, TxEnv, AccountInfo, U256, Bytecode, TransactTo};
use reth::revm::db::CacheDB;
use reth::revm::EVM;

use alloy_primitives::utils::parse_ether;

use async_trait::async_trait;
use std::str::FromStr;
use std::sync::Arc;

fn main() {
    Cli::<MyRethCliExt>::parse().run().unwrap();
}

struct MyRethCliExt;

impl RethCliExt for MyRethCliExt {
    type Node = RethCliEthExt;
}

#[derive(Debug, Clone, Copy, Default, clap::Args)]
struct RethCliEthExt {}

impl RethNodeCommandConfig for RethCliEthExt {
    fn extend_rpc_modules<Conf, Reth>(
        &mut self,
        _config: &Conf,
        _components: &Reth,
        rpc_components: RethRpcComponents<'_, Reth>,
    ) -> eyre::Result<()>
    where
        Conf: RethRpcConfig,
        Reth: RethNodeComponents,     
    {
        let eth_api = rpc_components.registry.eth_api().clone();
        let ext = EthTokenTax { eth_api: Arc::new(eth_api) };

        rpc_components.modules.merge_configured(ext.into_rpc())?;
        Ok(())
    }
}

#[derive(Debug)]
struct EthTokenTax<Eth> {
    eth_api: Arc<Eth>,
}

#[rpc(server, namespace = "eth")]
#[async_trait]
pub trait EthTokenTaxApi {
    /// Returns the buy and sell tax of given erc20 tokens
    #[method(name = "tokenTax")]
   async fn token_tax(&self, addresses: Vec<Address>) -> RpcResult<Vec<TaxCallResult>>;
}

#[async_trait]
impl<Eth> EthTokenTaxApiServer for EthTokenTax<Eth>
where
    Eth: EthTransactions + 'static
{
    async fn token_tax(&self, addresses: Vec<Address>) -> RpcResult<Vec<TaxCallResult>> {

        if addresses.is_empty() {
            return Err(ErrorObject::owned(
                -1,
                "No token provided",
                None::<()>,
            ));
        }
        // get env configuration
        let latest_block_id = BlockId::Number(BlockNumberOrTag::Latest);
        let (cfg, block_env, at) = self.eth_api.evm_env_at(latest_block_id).await?;
        
        let res = self.eth_api
            .spawn_with_state_at_block(at, move |state| {
            let mut env = Env { cfg, block: block_env, tx: TxEnv::default() };
            let mut db = CacheDB::new(StateProviderDatabase::new(state));
            
            // insert tax checker contract account in db
            let tax_checker_code = get_tax_checker_code();
            let code_hash = keccak256(&tax_checker_code);
            let tax_checker_bytecode = Bytecode::new_raw(tax_checker_code);
            let contract_account = AccountInfo::new(U256::from(0), 0, code_hash, tax_checker_bytecode);
            db.insert_account_info(tax_checker_address(), contract_account);

            // insert tax checker controller address in db
            let controller_account = AccountInfo::new(parse_ether("69").unwrap(), 0, KECCAK_EMPTY, Bytecode::default());
            db.insert_account_info(tax_checker_controller_address(), controller_account);
            
            env.tx.caller = tax_checker_controller_address();
            env.tx.gas_limit = 7000000;
            env.tx.gas_price = U256::from_str("100000000000").unwrap();

            let mut evm = EVM::with_env(env);
            evm.database(db);
            let mut results: Vec<TaxCallResult> = Vec::with_capacity(addresses.len());
            
            let mut addresses = addresses.into_iter().peekable();

            while let Some(token) = addresses.next() {

                let mut token_tax_info = TaxInfo::new(token);
                
                // get pair address. Expect that token LP contract is UniswapV2 Factory
                let pair_address = match token_tax_info.get_pair_address(&mut evm) {
                    Ok(pair) => pair,
                    Err(e) => {
                        results.push(TaxCallResult::CallError(e)); 
                        continue;
                    }
                };

                // insert the fake allowance to transfer tokens to the contract
                insert_fake_approval(token, pair_address, evm.db().unwrap());
                evm.env.tx.transact_to = TransactTo::Call(tax_checker_address());

                // getTax calldata
                let encoded_call = token_tax_info.encode_call(pair_address).into();
                evm.env.tx.data = encoded_call;

                // execute the call
                let ResultAndState { result, ..} = evm.transact()?;
                // handle the result
                if result.is_success() {
                    let output = result.into_output().unwrap_or_default();
                    token_tax_info.decode_success_call(output);
                    results.push(TaxCallResult::Success(token_tax_info));
                } else {
                    let revert = result.into_output().unwrap_or_default();
                    results.push(TaxCallResult::CallError(TaxCallError::CallingTaxError(token, revert.to_string())));
                };
            }


            Ok(results)
        })
        .await;

        res.map_err(|err| ErrorObject::from(err))
    }
}