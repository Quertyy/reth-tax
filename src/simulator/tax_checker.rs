use std::{str::FromStr, sync::Arc};

use super::{
    builder::SimulatorBuilder,
    constants::*,
    fork_factory::ForkFactory,
    helpers::map_location,
    simulator::{Simulator, TransactInfo},
};

use ethers::{prelude::*, utils::parse_ether, abi::parse_abi};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{Bytecode, U256 as rU256},
};
use reth::providers::{StateProvider, BlockNumReader};

#[allow(dead_code)]
pub async fn get_tax<Provider>(
    token_in: Address,
    pair: Address,
    fees: U256,
    provider: Arc<Provider>,
) -> Option<(U256, U256)> 
where
    Provider: BlockNumReader + StateProvider + 'static
{
    let cache_db: CacheDB<EmptyDB> = CacheDB::new(EmptyDB::default());
    let mut fork_factory = ForkFactory::new_sandbox_factory(provider.clone(), cache_db, None);
    inject_tax_checker_code(&mut fork_factory);
    insert_fake_approval(token_in, pair, &mut fork_factory);
    let builder = SimulatorBuilder::new(provider, fork_factory).await;
    
    let contract = BaseContract::from(parse_abi(&["function getTax(address,address,uint256)"]).unwrap());
    let tx_data = contract.encode("getTax", (pair, token_in, fees)).unwrap();

    let transact_info = TransactInfo {
        caller: tax_checker_controller_address(),
        to: tax_checker_address(),
        value: U256::from(0),
        data: tx_data,
    };

    let mut simulator = Simulator::new(builder).await;
    simulator.set_transact_info(transact_info).await;

    let output = match simulator.execute_commit() {
        Ok(output) => output,
        Err(_) => return None,
    };

    let tax_result = contract.decode_output("getTax", output);
    let (buy_tax, sell_tax) = match tax_result {
        Ok(d) => d,
        Err(_) => {
            return None;
        }
    };

    if buy_tax > U256::from(9970) || sell_tax > U256::from(9970) {
        return None;
    }
    Some((buy_tax, sell_tax))
}

#[allow(dead_code)]
pub fn insert_fake_approval(token: Address, pair: Address, fork_factory: &mut ForkFactory) {
    for i in 0..100 {
        let slot_new = map_location(U256::from(i), pair, tax_checker_address().0.into());
        fork_factory
            .insert_account_storage(
                token.0.into(),
                slot_new.into(),
                rU256::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639932").unwrap(),
            )
            .unwrap();
    }
}

#[allow(dead_code)]
pub fn inject_tax_checker_code(fork_factory: &mut ForkFactory) {
    let account = revm::primitives::AccountInfo::new(
        rU256::from(0),
        0,
        Bytecode::new_raw(get_tax_checker_code().0),
    );
    fork_factory.insert_account_info(tax_checker_address().0.into(), account);

    // setup braindance contract controller
    let account =
        revm::primitives::AccountInfo::new(parse_ether(69).unwrap().into(), 0, Bytecode::default());
    fork_factory.insert_account_info(tax_checker_controller_address().0.into(), account);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::Ws;
    use std::sync::Arc;
    use std::str::FromStr;

    async fn get_provider() -> Arc<Provider<Ws>> {
        let ws_url = "ws://localhost:8546";
        let ws_provider = Provider::<Ws>::connect(ws_url).await.expect("Could not connect to rpc");
        Arc::new(ws_provider);
        todo!()
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_get_token_taxes_univ2() {
        let provider = get_provider().await;

        let token0 = Address::from_str("0x1396D6F2e9056954DFc2775204bB3e2Eb8ab8a5B").unwrap();
        let pair = Address::from_str("0x4305BCC56Bb35FaF3edf13421F8D3167df32faB3").unwrap();

        let (buy_tax, sell_tax) = match get_tax(token0, pair, U256::from(997), provider).await {
            Some(d) => d,
            None => {
                return;
            }
        };
        assert_eq!(buy_tax, U256::from(100));
        assert_eq!(sell_tax, U256::from(100));
    }
}