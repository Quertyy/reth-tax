mod utils;
use utils::{get_tax_checker_code, tax_checker_address, tax_checker_controller_address};
use clap::Parser;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use reth::cli::{
    components::{RethNodeComponents, RethRpcComponents},
    config::RethRpcConfig,
    ext::{RethCliExt, RethNodeCommandConfig},
    Cli,
}; 

use reth::rpc::eth::{EthTransactions, error::EthApiError};
use reth::primitives::{BlockId, BlockNumberOrTag, keccak256};
use reth::revm::database::StateProviderDatabase;
use reth::revm::primitives::{Env, ResultAndState, TxEnv, AccountInfo, U256, Bytecode};
use reth::revm::db::CacheDB;
use reth::revm::EVM;

use alloy_primitives::Address;
use std::sync::Arc;
use async_trait::async_trait;

fn main() {}

struct MyRethCliExt;

impl RethCliExt for MyRethCliExt {
    type Node = RethCliEthExt;
}

#[derive(Debug, Clone, Copy, Default, clap::Args)]
struct RethCliEthExt {}

impl RethNodeCommandConfig for RethCliEthExt {
    fn extend_rpc_modules<Conf, Reth>(
        &mut self,
        config: &Conf,
        components: &Reth,
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

#[cfg_attr(not(test), rpc(server, namespace = "eth"))]
#[cfg_attr(test, rpc(server, client, namespace = "eth"))]
#[async_trait]
//#[async_trait::async_trait]
pub trait EthTokenTaxApi {
    /// Returns the number of transactions in the pool.
    #[method(name = "tokenTax")]
   async fn token_tax(&self, addresses: Vec<Address>) -> RpcResult<(u8, u8)>;
}

#[async_trait]
impl<Eth> EthTokenTaxApiServer for EthTokenTax<Eth>
where
    Eth: EthTransactions + 'static
{
    async fn token_tax(&self, addresses: Vec<Address>) -> RpcResult<(u8, u8)> {
        if addresses.is_empty() {
            return Err(EthApiError::InvalidParams(String::from("No token address provided.")).into())
        }
        let latest_block_id = BlockId::Number(BlockNumberOrTag::Latest);
        let (cfg, mut block_env, at) = self.eth_api.evm_env_at(latest_block_id).await?;
        
        let _ = self.eth_api
            .spawn_with_state_at_block(at, |state| {
            let env = Env { cfg, block: block_env, tx: TxEnv::default() };
            let mut db = CacheDB::new(StateProviderDatabase::new(state));

            let tax_checker_code = get_tax_checker_code();
            let code_hash = keccak256(&tax_checker_code);
            let tax_checker_bytecode = Bytecode::new_raw(tax_checker_code);
            let account = AccountInfo::new(U256::from(69), 0, code_hash, tax_checker_bytecode);
            db.insert_account_info(tax_checker_address(), account);
            let mut evm = EVM::with_env(env);
            evm.database(db);

            
            Ok(())
        })
        .await;
    
        Ok((1, 1))
    }
}