use ethers::providers::{Provider, Ws};
use std::sync::Arc;

use super::fork_factory::ForkFactory;

pub struct SimulatorBuilder {
    pub wss_provider: Arc<Provider<Ws>>,
    pub fork_factory: ForkFactory,
}

impl SimulatorBuilder {
    pub async fn new(wss_provider: Arc<Provider<Ws>>, fork_factory: ForkFactory) -> Self {
        Self { wss_provider, fork_factory }
    }
}
