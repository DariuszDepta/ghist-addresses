use cosmwasm_std::testing::MockApi;
use cosmwasm_std::{instantiate2_address, Api};

fn main() {
    let api = MockApi::default();
    let human_addr = api.addr_make("creator");
    let canonical_addr = api.addr_canonicalize(human_addr.as_str()).unwrap();
    let checksum = &[87; 32];
    let salt = &[1; 5];
    let contract_address = instantiate2_address(checksum, &canonical_addr, salt).unwrap();
    let result = api.addr_humanize(&contract_address);
    println!("{:?}", result);
}
