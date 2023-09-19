# Address handling using bech32

This is a proposal how to adjust address handling in MockApi.

## Comments to proposal

After brainstorming about this change with @webmaster128 and @chipshort we agreed,
that the simplest, safest and the closest to real-life blockchain way of handling addresses
in tests will be using Bech32 encoding. It means, that all user addresses and contract
addresses will be either in canonical form or Bech32 encoded.

## Planned changes

Implementation of the trait `cosmwasm_std::Api` for `cosmwasm_std::testing::MockApi` will be
adjusted in the extent described below.

### Function `addr_validate`

This function will probably stay untouched. The validation process checks if the input address
can be canonicalized and then humanized, and the result is compared with the input.

It means, that when the user passes valid Bech32 or Bech32m address
with predefined prefix (see `Bech32 prefix` section below) to this function,
it will be properly canonicalized. Canonical address can be always humanized,
so the whole validation process will always success.

In case the user passes an invalid address (not Bech32 encoded), the canonicalization
will reject it with an error, and then the whole validation fails. 

### Function `addr_canonicalize`

Canonicalization will use Bech32 decoder with predefined prefix. If decoding fails,
it means that the provided address is invalid and an error will be returned.
After change, this function will accept only addresses in Bech32 or Bech32m encoding
having predefined prefix (see `Bech32 prefix` section below).

### Function `addr_humanize`

This function will use Bech32 encoder to generate humanized address
with predefined prefix (see `Bech32 prefix` section below).

### Bech32 prefix

`cosmwasm_std::testing::MockApi` during instantiation will have a predefined prefix set.
The default prefix used in Bech32 encoding/decoding will be **cosmwasm**, but it can be set
to any reasonable value. So the user may create addresses like these in real blockchain, 
e.g. **juno1v82su97skv6ucfqvuvswe0t5fph7pfsrtraxf0x33d8ylj5qnrysdvkc95**

### `Addr` struct

We will provide utility function named **_hashed_**, to be used only for testing purposes
(just like **_unchecked_**) for creating Bech32 addresses from any provided input.
The _unchecked_ function will be deprecated, so the proper ways of creating `Addr` will be:
- using `let checked: Addr = Addr::hashed(input)`,
- using `let checked: Addr = deps.api.addr_validate(input)?`, or
- using `let checked: Addr = deps.api.addr_humanize(canonical_addr)?`.

## Implementation plan

Adding function `Addr::hashed` is non-breaking change, under condition, that bech32 crate compiles to wasm32
(can be implemented immediately).

Changing the behaviour of functions `addr_canonicalize` and `addr_humanize` is a breaking change.
These changes must be postponed to version 2.0. All test cases im `cosmwasm` and tests for contracts (cw-plus)
have to be reviewed and adjusted.

Adding Bech32 prefix to MockApi seems to be a non-breaking change and can be implemented immediately. 