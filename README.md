# Address in CosmWasm

This issue is well-known and was reported in [#1648](https://github.com/CosmWasm/cosmwasm/issues/1648).

To reproduce the error locally, just create a Rust app with following dependencies:

```shell
$ cat Cargo.toml
[package]
name = "addresses"
version = "0.0.1"
edition = "2021"

[dependencies]
cosmwasm-std = "1.2.8"
cw-multi-test = { version = "0.20.0", features = ["cosmwasm_1_2"] }
```

and single source file:

```shell
$ cat src/main.rs
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
```

The result should be:

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/addresses`
Err(GenericErr { msg: "Invalid input: canonical address length not correct" })
```

