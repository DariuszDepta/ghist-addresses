use cosmwasm_std::{instantiate2_address, Api};
use cw_multi_test::App;

fn main() {
    let app = App::default();
    let human_addr = app.api().addr_make("creator");
    let canonical_addr = app.api().addr_canonicalize(human_addr.as_str()).unwrap();
    let checksum = &[87; 32];
    let salt = &[1; 5];
    let contract_address = instantiate2_address(checksum, &canonical_addr, salt).unwrap();
    let result = app.api().addr_humanize(&contract_address);
    println!("{:?}", result);
}
