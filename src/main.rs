#![doc = include_str!("../README.md")]

use cosmwasm_std::{Api, HexBinary, instantiate2_address};
use crate::mock::MockApi;

pub mod mock;

/// Checksum used in `instantiate2_address` function.
pub const CHECKSUM: &str = "13a1fc994cc6d1c81b746ee0c0ff6f90043875e0bf1d9be6b7d779fc978dc2a5";

/// Address of a creator used in `instantiate2_address` function.
///
/// The canonical value of this address is: `9999999999aaaaaaaaaabbbbbbbbbbcccccccccc`, and the bech32 prefix is `purple`.
pub const CREATOR: &str = "purple1nxvenxve42424242hwamhwamenxvenxvhxf2py";

/// Salt used in used in `instantiate2_address` function.
pub const SALT: &str = "61";

/// Generates humanized contract address, like described in this [issue](https://github.com/CosmWasm/cosmwasm/issues/1648).
pub fn main() {
    // initialize MockApi, the chain prefix used in test is "purple",
    // but every different chain name can be used, e.g. "juno", "osmosis", etc.
    let api = MockApi::new_with_bech32_prefix("purple");

    // initialize checksum, creator's canonical address and salt for contract address generation
    let checksum = HexBinary::from_hex(CHECKSUM).unwrap();
    let creator = api.addr_canonicalize(CREATOR).unwrap();
    let salt = HexBinary::from_hex(SALT).unwrap();

    // generate contract address
    let contract_addr = instantiate2_address(checksum.as_slice(), &creator, salt.as_slice()).unwrap();

    // humanize the contract address...
    let addr = api.addr_humanize(&contract_addr).unwrap();

    // ...and the generated address should look like: purple1t6r960j945lfv8mhl4mage2rg97w63xeynwrupum2s2l7em4lprs9ce5hk
    println!("{}", addr);
}
