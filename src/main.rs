mod simulator;

use clap::Parser;
use ethers::{types::Address, abi::token};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use reth::cli::{
    components::{RethNodeComponents, RethRpcComponents},
    config::RethRpcConfig,
    ext::{RethCliExt, RethNodeCommandConfig},
    Cli,
};

fn main() {
    Cli::<MyRethCliExt>::parse().run().unwrap();
}

/// The type that tells the reth CLI what extensions to use
struct MyRethCliExt;

impl RethCliExt for MyRethCliExt {
    /// This tells the reth CLI to install the `txpool` rpc namespace via `RethCliTxpoolExt`
    type Node = RethCliEthExt;
}

/// Our custom cli args extension that adds one flag to reth default CLI.
#[derive(Debug, Clone, Copy, Default, clap::Args)]
struct RethCliEthExt {
    /// CLI flag to enable the eth extension namespace
    #[clap(long)]
    pub enable_ext: bool,
}

impl RethNodeCommandConfig for RethCliEthExt {
    fn extend_rpc_modules<Conf, Reth>(
            &mut self,
            _config: &Conf,
            _components: &Reth,
            _rpc_components: RethRpcComponents<'_, Reth>,
    ) -> eyre::Result<()>
    where
        Conf: RethRpcConfig,
        Reth: RethNodeComponents, 
    {
        Ok(())
    }
}

#[cfg_attr(not(test), rpc(server, namespace = "ethExt"))]
#[cfg_attr(test, rpc(server, client, namespace = "ethExt"))]
pub trait EthExtApi {
    /// Returns the number of transactions in the pool.
    #[method(name = "getTokenTax")]
    fn token_tax(&self, token_addresses: Vec<Address>) -> RpcResult<Option<(u8, u8)>>;
}

pub struct EthExt;

impl EthExtApiServer for EthExt {
    fn token_tax(&self, token_addresses:Vec<Address>) -> RpcResult<Option<(u8,u8)>> {
        for token in token_addresses.iter() {
            
        }
        todo!()
    }
}