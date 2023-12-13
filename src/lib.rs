mod simulator;

use clap::Parser;
use ethers::types::Address;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use reth::rpc::eth::error::EthResult;

use reth::{primitives::Block, providers::BlockReaderIdExt};

#[rpc(server, namespace = "ethExt")]
pub trait EthExtApi {
    /// Returns the buy and sell taxes of given tokens
    #[method(name = "getTokenTax")]
    fn token_tax(&self, token_addresses: Vec<Address>) -> EthResult<Option<Block>>;
}

pub struct EthExt<Provider> {
    pub provider: Provider,
}

impl<Provider> EthExtApiServer for EthExt<Provider> 
where 
    Provider: BlockReaderIdExt + 'static,
{
    fn token_tax(&self, token_addresses:Vec<Address>) -> EthResult<Option<Block>> {
        for token in token_addresses.iter() {
            
        }
        let block = self.provider.block_by_number(0)?;
        Ok(block)
    }
}