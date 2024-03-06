#![allow(unused)]

use cosmwasm_std::testing::MockApi;
use cosmwasm_std::Api;

fn generate_human_readable_address() {
    let api = MockApi::default();
    let human_addr = api.addr_make("creator");
    println!("\nHuman readable address: {}", human_addr.as_str());
}

fn generate_canonical_address() {
    let api = MockApi::default();
    let human_addr = api.addr_make("creator");
    println!("\nHuman readable address: {}", human_addr.as_str());
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

fn main() {
    // generate_human_readable_address();
    generate_canonical_address();
}
