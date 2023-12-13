use std::sync::Arc;
use reth::providers::{StateProvider, BlockNumReader};
use super::fork_factory::ForkFactory;

pub struct SimulatorBuilder<Provider> 
where
    Provider: StateProvider + BlockNumReader + 'static    
{
    pub provider: Arc<Provider>,
    pub fork_factory: ForkFactory,
}

impl<Provider> SimulatorBuilder<Provider>
where
    Provider: StateProvider + BlockNumReader + 'static
{
    pub async fn new(provider: Arc<Provider>, fork_factory: ForkFactory) -> Self {
        Self { provider, fork_factory }
    }
}
