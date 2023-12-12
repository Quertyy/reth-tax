use ethers::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub fn map_location(slot: U256, key: Address, key_after: Address) -> U256 {
    let key_slot_hash: U256 =
        ethers::utils::keccak256(abi::encode(&[abi::Token::Address(key), abi::Token::Uint(slot)]))
            .into();

    let slot: U256 = ethers::utils::keccak256(abi::encode(&[
        abi::Token::Address(key_after),
        abi::Token::Uint(key_slot_hash),
    ]))
    .into();

    slot
}

pub fn get_current_unix_time_seconds() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_epoch.as_secs()
}