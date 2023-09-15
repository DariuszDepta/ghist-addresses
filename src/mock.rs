//! # Proposed MockApi implementation

use bech32::{decode, FromBase32, ToBase32, Variant};
use cosmwasm_std::{
    Addr, Api, CanonicalAddr, RecoverPubkeyError, StdError, StdResult, VerificationError,
};

/// Default bech32 prefix used in tests.
pub const BECH32_PREFIX: &str = "mockapi";

/// Proposed MockApi.
#[derive(Clone)]
pub struct MockApi {
    /// The bech32 prefix used to canonicalize the address.
    bech32_prefix: String,
    /// The bech32 prefix used to encode addresses that are not in bech32 format, like "foobar".
    bech32_internal_prefix: String,
}

impl Default for MockApi {
    /// Creates [MockApi] with default bech32 prefix.
    fn default() -> Self {
        Self {
            bech32_prefix: BECH32_PREFIX.to_string(),
            bech32_internal_prefix: BECH32_PREFIX.to_string().chars().rev().collect(),
        }
    }
}

impl MockApi {
    /// Creates [MockApi] with user defined bech32 prefix..
    pub fn new_with_bech32_prefix(bech32_prefix: &str) -> Self {
        Self {
            bech32_prefix: bech32_prefix.to_string(),
            bech32_internal_prefix: bech32_prefix.chars().rev().collect(),
        }
    }
}

impl Api for MockApi {
    /// Validate the address.
    /// This function is just a copy of the original from cosmwasm_std::testing::MockApi.
    fn addr_validate(&self, input: &str) -> StdResult<Addr> {
        let canonical = self.addr_canonicalize(input)?;
        let normalized = self.addr_humanize(&canonical)?;
        if input != normalized {
            return Err(StdError::generic_err(
                "Address not normalized",
            ));
        }
        Ok(Addr::unchecked(input))
    }

    /// Canonicalizes the address.
    ///
    /// If `input` is bech32 address, with the same prefix as defined in [MockApi],
    /// then just unwrap the data from this address and store and return in `CanonicalAddr`.
    ///
    /// If `input` is bech32 address, but with different prefix as defined in [MockApi],
    /// then return an input error.
    ///
    /// If `input` is **NOT** a bech32 address, then wrap it in bech32 format using internal
    /// bech32 prefix, different from this configured in [MockApi].
    /// The wrapped value is now the _body_ of the canonical address.
    ///
    fn addr_canonicalize(&self, input: &str) -> StdResult<CanonicalAddr> {
        if let Ok((prefix, data, variant)) = decode(input) {
            if prefix == self.bech32_prefix && variant == Variant::Bech32 {
                if let Ok(bytes) = Vec::<u8>::from_base32(&data) {
                    return Ok(bytes.into());
                }
            }
        } else if let Ok(data) = bech32::encode(
            &self.bech32_internal_prefix,
            input.to_lowercase().as_bytes().to_base32(),
            Variant::Bech32,
        ) {
            return Ok(data.as_bytes().into());
        }
        Err(StdError::generic_err("Invalid input"))
    }

    /// Humanizes the canonical address.
    ///
    /// If `canonical` is a bech32 wrapped value with internal prefix (which means that before
    /// canonicalization it was just a simple address like "foobar") then to humanize it,
    /// just unwrap the value stored in bech32 format, so the result will be like "foobar".
    ///
    /// If `canonical` is a bech32 wrapped value with other prefix than internal,
    /// then return an error.
    ///
    /// If `canonical` is **NOT** a bech32 value, than wrap it in bech32 format
    /// using prefix configured in [MockApi], so the result will be like "juno129shy4384rhg...".
    ///
    fn addr_humanize(&self, canonical: &CanonicalAddr) -> StdResult<Addr> {
        if let Ok((prefix, data, variant)) = decode(&String::from_utf8_lossy(canonical)) {
            if prefix == self.bech32_internal_prefix && variant == Variant::Bech32 {
                if let Ok(bytes) = Vec::<u8>::from_base32(&data) {
                    return Ok(Addr::unchecked(String::from_utf8_lossy(&bytes)));
                }
            }
        } else if let Ok(data) = bech32::encode(
            &self.bech32_prefix,
            canonical.as_slice().to_base32(),
            Variant::Bech32,
        ) {
            return Ok(Addr::unchecked(data));
        }
        Err(StdError::generic_err("Invalid canonical address"))
    }

    fn secp256k1_verify(
        &self,
        _message_hash: &[u8],
        _signature: &[u8],
        _public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        todo!()
    }

    fn secp256k1_recover_pubkey(
        &self,
        _message_hash: &[u8],
        _signature: &[u8],
        _recovery_param: u8,
    ) -> Result<Vec<u8>, RecoverPubkeyError> {
        todo!()
    }

    fn ed25519_verify(
        &self,
        _message: &[u8],
        _signature: &[u8],
        _public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        todo!()
    }

    fn ed25519_batch_verify(
        &self,
        _messages: &[&[u8]],
        _signatures: &[&[u8]],
        _public_keys: &[&[u8]],
    ) -> Result<bool, VerificationError> {
        todo!()
    }

    fn debug(&self, _message: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_validate_works() {
        let api = MockApi::default();

        // valid: address is minimum 3 characters long and contains only small caps
        assert_eq!("foo", api.addr_validate("foo").unwrap());
        assert_eq!("foobar123", api.addr_validate("foobar123").unwrap());

        // invalid: address is not normalized
        api.addr_validate("Foobar123").unwrap_err();
        api.addr_validate("FOOBAR123").unwrap_err();
    }

    #[test]
    fn addr_canonicalize_works() {
        let api = MockApi::default();

        api.addr_canonicalize("foobar123").unwrap();

        // addr_canonicalize is case insensitive
        assert_eq!(
            api.addr_canonicalize("foo123").unwrap(),
            api.addr_canonicalize("FOO123").unwrap()
        );

        let canonical = api.addr_canonicalize("f").unwrap();
        assert_eq!("6970616B636F6D3176636778327A7334", canonical.to_string());
        assert_eq!(
            "ipakcom1vcgx2zs4",
            String::from_utf8_lossy(canonical.as_slice())
        );

        let canonical = api.addr_canonicalize("").unwrap();
        assert_eq!("6970616B636F6D3173716E686B34", canonical.to_string());
        assert_eq!(
            "ipakcom1sqnhk4",
            String::from_utf8_lossy(canonical.as_slice())
        );

        let canonical = api.addr_canonicalize("some-extremely-long-address-also-supported-by-this-api-some-extremely-long-address-also-supported-by-this-api-some-extremely-long-address-also-supported-by-this-api").unwrap();
        assert_eq!("6970616B636F6D317764686B366566647634753867756E3964346A6B633766646433686B7565656476396A7867756E397764656A366374767764686A36756D347770637837756E3576346A7A36636E65393436787336746E39347368713666647764686B366566647634753867756E3964346A6B633766646433686B7565656476396A7867756E397764656A366374767764686A36756D347770637837756E3576346A7A36636E65393436787336746E39347368713666647764686B366566647634753867756E3964346A6B633766646433686B7565656476396A7867756E397764656A366374767764686A36756D347770637837756E3576346A7A36636E65393436787336746E393473687136677433716B3972", 
                   canonical.to_string());
        assert_eq!(
            "ipakcom1wdhk6efdv4u8gun9d4jkc7fdd3hkueedv9jxgun9wdej6ctvwdhj6um4wpcx7un5v4jz6cne946xs6tn94shq6fdwdhk6efdv4u8gun9d4jkc7fdd3hkueedv9jxgun9wdej6ctvwdhj6um4wpcx7un5v4jz6cne946xs6tn94shq6fdwdhk6efdv4u8gun9d4jkc7fdd3hkueedv9jxgun9wdej6ctvwdhj6um4wpcx7un5v4jz6cne946xs6tn94shq6gt3qk9r",
            String::from_utf8_lossy(canonical.as_slice())
        );
    }

    #[test]
    fn canonicalize_and_humanize_restores_original() {
        let api = MockApi::new_with_bech32_prefix("juno");

        // simple
        let original = String::from("shorty");
        let canonical = api.addr_canonicalize(&original).unwrap();
        let recovered = api.addr_humanize(&canonical).unwrap();
        assert_eq!(recovered, original);
        let canonical = api.addr_canonicalize(recovered.as_ref()).unwrap();
        let recovered = api.addr_humanize(&canonical).unwrap();
        assert_eq!(recovered, original);

        // normalizes input
        let original = String::from("CosmWasmChef");
        let canonical = api.addr_canonicalize(&original).unwrap();
        let recovered = api.addr_humanize(&canonical).unwrap();
        assert_eq!(recovered, "cosmwasmchef");
        let canonical = api.addr_canonicalize(recovered.as_ref()).unwrap();
        let recovered = api.addr_humanize(&canonical).unwrap();
        assert_eq!(recovered, "cosmwasmchef");

        // Long input (Juno contract address)
        let original =
            String::from("juno1v82su97skv6ucfqvuvswe0t5fph7pfsrtraxf0x33d8ylj5qnrysdvkc95");
        let canonical = api.addr_canonicalize(&original).unwrap();
        let recovered = api.addr_humanize(&canonical).unwrap();
        assert_eq!(recovered, original);
        let canonical = api.addr_canonicalize(recovered.as_ref()).unwrap();
        let recovered = api.addr_humanize(&canonical).unwrap();
        assert_eq!(recovered, original);
    }

    #[test]
    fn can_be_tricked() {
        let api = MockApi::default();

        // create nested bech32 address
        let inner_addr = bech32::encode(
            &api.bech32_internal_prefix,
            b"asdf".to_base32(),
            Variant::Bech32,
        )
        .unwrap();
        let outer_addr = bech32::encode(
            &api.bech32_prefix,
            inner_addr.as_bytes().to_base32(),
            Variant::Bech32,
        )
        .unwrap();

        // now canonicalize and humanize again
        let result = api
            .addr_humanize(&api.addr_canonicalize(&outer_addr).unwrap())
            .unwrap();

        // the result is different from the input
        assert_ne!(outer_addr, result);
    }

    #[test]
    fn can_handle_inner_addr() {
        let api = MockApi::default();

        // create bech32 address with different variant
        let inner_addr = bech32::encode(
            &api.bech32_internal_prefix,
            b"asdf".to_base32(),
            Variant::Bech32m,
        )
        .unwrap();
        // nest it as data inside an address with correct variant
        let outer_addr = bech32::encode(
            &api.bech32_prefix,
            inner_addr.as_bytes().to_base32(),
            Variant::Bech32,
        )
        .unwrap();

        // now canonicalize and humanize again
        let result = api
            .addr_humanize(&api.addr_canonicalize(&outer_addr).unwrap())
            .unwrap();

        assert_eq!(outer_addr, result);
    }
}
