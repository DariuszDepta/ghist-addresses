# Address in CosmWasm

This issue is well-known and was reported in [#1648](https://github.com/CosmWasm/cosmwasm/issues/1648).

The problem was firstly resolved in version **2.0.0-rc.0** of `cosmwasm-std`. It will be officially
available in version **2.0.0** of `cosmwasm-std` and version **2.0.0** of `cw-multi-test`.

To reproduce the original error locally, just create a Rust app with following dependencies:

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

and a single source file:

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

To check the fix in `cosmwasm-std@2.0.0-rc.0` create a simple Rust app with the following dependencies:

```shell
$cat Cargo.toml
[package]
name = "addresses"
version = "0.0.1"
edition = "2021"

[dependencies]
cosmwasm-std = "2.0.0-rc.0"
```

and a single source file:

```shell
$ cat src/main.rs
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
```

The result should be:

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/addresses`
Ok(Addr("cosmwasm19v9fz5q2v9xkkjr7u9jet95pdz8xncq8h5s6xqesph2fpe97dgjs0p38pf"))
```

After publishing `cw-multi-test@2.0.0-rc.0` we will provide a working example using `cw-multi-test`.

