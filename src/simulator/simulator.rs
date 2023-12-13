use std::str::FromStr;

use anyhow::Result;
use ethers::{
    core::types::Bytes,
    providers::Middleware,
    types::{Address, U256},
};
use revm::{
    primitives::{
        Address as rAddress, Bytes as rBytes, ExecutionResult, Output, TransactTo, U256 as rU256,
    },
    EVM,
};
use reth::providers::{StateProvider, BlockNumReader};

use super::{builder::SimulatorBuilder, fork_db::ForkDB, DatabaseError};
use super::get_current_unix_time_seconds;

#[derive(Debug, thiserror::Error)]
pub enum SimulatorErrors {
    #[error("{0:#?}")]
    EVMError(revm::primitives::EVMError<DatabaseError>),
    #[error("Transaction reverts: {0:?}")]
    TransactRevert(Bytes),
    #[error("Transaction halted")]
    TransactHalt(Bytes),
}

pub struct TransactInfo {
    pub caller: Address,
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
}

pub struct Simulator<Provider> 
where
    Provider: StateProvider + BlockNumReader + 'static
{
    pub builder: SimulatorBuilder<Provider>,
    pub evm: EVM<ForkDB>,
}

impl<Provider> Simulator<Provider> 
where 
    Provider: StateProvider + BlockNumReader +'static
{
    pub async fn new(builder: SimulatorBuilder<Provider>) -> Self {
        let current_block = builder.provider.last_block_number().unwrap();
        let mut evm: revm::EVM<ForkDB> = revm::EVM::new();
        let sandbox = builder.fork_factory.new_sandbox_fork();
        evm.database(sandbox);
        evm.env.block.number = rU256::from(current_block);
        evm.env.block.timestamp = rU256::from(get_current_unix_time_seconds());
        evm.env.block.basefee = rU256::from(1000000);
        evm.env.block.coinbase =
            rAddress::from_str("0xDecafC0FFEe15BAD000000000000000000000000").unwrap();

        evm.env.tx.gas_limit = 7000000;
        evm.env.tx.gas_price = rU256::from(1000000000);

        Self { builder, evm }
    }

    pub async fn set_transact_info(&mut self, transact_info: TransactInfo) {
        self.evm.env.tx.value = transact_info.value.into();
        self.evm.env.tx.caller = transact_info.caller.0.into();
        self.evm.env.tx.transact_to = TransactTo::Call(transact_info.to.0.into());
        self.evm.env.tx.data = transact_info.data.0;
    }

    pub fn execute_commit(&mut self) -> Result<rBytes, SimulatorErrors> {
        let result: ExecutionResult = match self.evm.transact_commit() {
            Ok(result) => result,
            Err(err) => return Err(SimulatorErrors::EVMError(err)),
        };

        let output = match result {
            ExecutionResult::Success { output, .. } => match output {
                Output::Call(o) => o,
                Output::Create(o, _) => o,
            },
            ExecutionResult::Revert { gas_used: _, output } => {
                return Err(SimulatorErrors::TransactRevert(output.into()));
            }
            ExecutionResult::Halt { .. } => {
                return Err(SimulatorErrors::TransactHalt(b"0x".into()));
            }
        };

        Ok(output)
    }
}
