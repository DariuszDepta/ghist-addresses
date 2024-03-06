#![allow(unused)]

mod end;

use crate::end::end;
use cosmwasm_std::instantiate2_address;
use cosmwasm_std::testing::MockApi;
use cosmwasm_std::Api;

fn generate_human_readable_address() {
    let api = MockApi::default();
    let human_addr = api.addr_make("creator");
    println!("\nHuman-readable address: {}", human_addr);
}

fn generate_canonical_address() {
    let api = MockApi::default();
    let human_addr = api.addr_make("creator");
    println!("\nHuman-readable address: {}", human_addr);
    let canonical_addr = api.addr_canonicalize(human_addr.as_str()).unwrap();
    println!("\nCanonical address: {}", canonical_addr.to_string());
    println!(
        "\nCanonical address as bytes: {:?}",
        canonical_addr.as_slice()
    );
    println!(
        "\nCanonical address is always {} bytes long",
        canonical_addr.as_slice().len()
    );
}

fn predictable_contract_address() {
    let api = MockApi::default();
    let checksum = &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];
    let creator = api
        .addr_canonicalize(api.addr_make("creator").as_str())
        .unwrap();
    let salt = &[100, 101, 102];
    let canonical_addr = instantiate2_address(checksum, &creator, salt).unwrap();
    let human_addr = api.addr_humanize(&canonical_addr).unwrap();
    println!("\nHuman-readable address: {}", human_addr);
}

fn generate_human_readable_address_with_prefix(hrp: &'static str) {
    let api = MockApi::default().with_prefix(hrp);
    let human_addr = api.addr_make("creator");
    println!("\nHuman-readable address: {}", human_addr);
}

fn round_trip() {
    let api = MockApi::default().with_prefix("osmo");
    let human_addr = api.addr_make("bobby");
    println!("\nHuman-readable address: {}", human_addr.as_str());
    let canonical_addr = api.addr_canonicalize(human_addr.as_str()).unwrap();
    let human_addr = api.addr_humanize(&canonical_addr).unwrap();
    let canonical_addr = api.addr_canonicalize(human_addr.as_str()).unwrap();
    let human_addr = api.addr_humanize(&canonical_addr).unwrap();
    let validated_addr = api.addr_validate(human_addr.as_str()).unwrap();
    println!("     Validated address: {}", validated_addr);
}

// tweak Cargo.toml before you uncomment this
/*
fn addresses_in_tests() {
    use cw_multi_test::App;

    let app = App::default();
    let human_addr = app.api().addr_make("creator");
    println!("Human-readable address: {}", human_addr);
    let canonical_addr = app.api().addr_canonicalize(human_addr.as_str()).unwrap();
    println!("     Canonical address: {}", canonical_addr.to_string());
    let human_addr = app.api().addr_humanize(&canonical_addr).unwrap();
    println!("Human-readable address: {}", human_addr);
    let validated_addr = app.api().addr_validate(human_addr.as_str()).unwrap();
    println!("     Validated address: {}", validated_addr);
}
*/

fn main() {
    generate_human_readable_address();
    // generate_canonical_address();
    // predictable_contract_address();
    // generate_human_readable_address_with_prefix("juno");
    // round_trip();
    // addresses_in_tests();
    // end();
}
