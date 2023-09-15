use bech32::{decode, encode, FromBase32, ToBase32, Variant};

/// Default bech32 prefix.
const DEFAULT_PREFIX: &str = "cosmwasm";

#[derive(Copy, Clone)]
pub struct Bech {
    /// Default or user-defined bech32 prefix.
    prefix: &'static str,
    /// Variant of the bech32 encoding.
    variant: Variant,
}

impl Default for Bech {
    /// Creates [Bech] with the default prefix.
    fn default() -> Self {
        Self {
            prefix: DEFAULT_PREFIX,
            variant: Variant::Bech32,
        }
    }
}

impl Bech {
    ///
    pub fn new_b32(prefix: &'static str) -> Self {
        Self {
            prefix,
            variant: Variant::Bech32,
        }
    }

    ///
    pub fn new_b32m(prefix: &'static str) -> Self {
        Self {
            prefix,
            variant: Variant::Bech32m,
        }
    }

    ///
    pub fn canonicalize(&self, input: &str) -> Option<Vec<u8>> {
        if let Ok((prefix, _, _)) = decode(input) {
            if prefix == self.prefix || prefix == self.rev_prefix() {
                return Some(input.as_bytes().to_vec());
            }
        } else if let Ok(data) = encode(
            &self.rev_prefix(),
            input.to_lowercase().as_bytes().to_base32(),
            self.variant,
        ) {
            return Some(data.as_bytes().to_vec());
        }
        None
    }

    ///
    pub fn humanize(&self, canonical: &[u8]) -> Option<String> {
        let canonical_str = String::from_utf8_lossy(canonical);
        if let Ok((prefix, decoded, _)) = decode(canonical_str.as_ref()) {
            if prefix == self.prefix {
                return Some(canonical_str.to_string());
            } else if prefix == self.rev_prefix() {
                if let Ok(bytes) = Vec::<u8>::from_base32(&decoded) {
                    return Some(String::from_utf8_lossy(&bytes).to_string());
                }
            }
        } else if let Ok(encoded) = encode(self.prefix, canonical.to_base32(), self.variant) {
            return Some(encoded);
        }
        None
    }

    ///
    fn rev_prefix(&self) -> String {
        self.prefix.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JUNO_ADDRESS: &str = "juno1v82su97skv6ucfqvuvswe0t5fph7pfsrtraxf0x33d8ylj5qnrysdvkc95";

    fn hex(input: &[u8]) -> String {
        input.iter().map(|byte| format!("{:02X}", byte)).collect()
    }

    fn utf8(input: &[u8]) -> String {
        String::from_utf8_lossy(input).to_string()
    }

    #[test]
    fn canonicalize_works() {
        let bech = Bech::default();
        let canonical = bech.canonicalize("foobar123").unwrap();
        assert_eq!("msawmsoc1vehk7cnpwgcnyvclw6y9j", utf8(&canonical));
        assert_eq!(
            "6D7361776D736F63317665686B37636E707767636E7976636C773679396A",
            hex(&canonical)
        );
    }

    #[test]
    fn canonicalize_is_case_insensitive() {
        let bech = Bech::default();
        assert_eq!(
            bech.canonicalize("foo123").unwrap(),
            bech.canonicalize("FOO123").unwrap()
        );
    }

    #[test]
    fn canonicalize_empty_address_works() {
        let bech = Bech::default();
        let canonical = bech.canonicalize("").unwrap();
        assert_eq!("msawmsoc1d2j362", utf8(&canonical));
        assert_eq!("6D7361776D736F633164326A333632", hex(&canonical));
    }

    #[test]
    fn canonicalize_short_address_works() {
        let bech = Bech::default();
        let canonical = bech.canonicalize("a").unwrap();
        assert_eq!("msawmsoc1vylhjcfd", utf8(&canonical));
        assert_eq!("6D7361776D736F633176796C686A636664", hex(&canonical));
    }

    #[test]
    fn canonicalize_long_address_works() {
        let bech = Bech::default();
        let canonical = bech.canonicalize("some-extremely-long-address-also-supported-by-this-api-some-extremely-long-address-also-supported-by-this-api-some-extremely-long-address-also-supported-by-this-api").unwrap();
        assert_eq!("msawmsoc1wdhk6efdv4u8gun9d4jkc7fdd3hkueedv9jxgun9wdej6ctvwdhj6um4wpcx7un5v4jz6cne946xs6tn94shq6fdwdhk6efdv4u8gun9d4jkc7fdd3hkueedv9jxgun9wdej6ctvwdhj6um4wpcx7un5v4jz6cne946xs6tn94shq6fdwdhk6efdv4u8gun9d4jkc7fdd3hkueedv9jxgun9wdej6ctvwdhj6um4wpcx7un5v4jz6cne946xs6tn94shq6gvu3g2s", utf8(&canonical));
        assert_eq!("6D7361776D736F63317764686B366566647634753867756E3964346A6B633766646433686B7565656476396A7867756E397764656A366374767764686A36756D347770637837756E3576346A7A36636E65393436787336746E39347368713666647764686B366566647634753867756E3964346A6B633766646433686B7565656476396A7867756E397764656A366374767764686A36756D347770637837756E3576346A7A36636E65393436787336746E39347368713666647764686B366566647634753867756E3964346A6B633766646433686B7565656476396A7867756E397764656A366374767764686A36756D347770637837756E3576346A7A36636E65393436787336746E39347368713667767533673273", hex(&canonical));
    }

    #[test]
    fn double_canonicalize_of_short_address_has_no_effect() {
        let bech = Bech::default();
        let addr1 = "foobar123";
        let canonical1 = bech.canonicalize(addr1).unwrap();
        let addr2 = String::from_utf8_lossy(&canonical1);
        let canonical2 = bech.canonicalize(addr2.as_ref()).unwrap();
        assert_eq!(canonical1, canonical2);
    }

    #[test]
    fn double_canonicalize_of_long_address_has_no_effect() {
        let bech = Bech::new_b32("juno");
        let addr1 = JUNO_ADDRESS;
        let canonical1 = bech.canonicalize(addr1).unwrap();
        let addr2 = String::from_utf8_lossy(&canonical1);
        let canonical2 = bech.canonicalize(addr2.as_ref()).unwrap();
        assert_eq!(canonical1, canonical2);
    }

    #[test]
    fn double_humanize_of_short_address_has_no_effect() {
        let bech = Bech::new_b32("juno");
        let canonical1 = "foobar123".as_bytes();
        let humanized1 = bech.humanize(canonical1).unwrap();
        let canonical2 = humanized1.as_bytes();
        let humanized2 = bech.humanize(canonical2).unwrap();
        assert_eq!(humanized1, humanized2);
    }

    #[test]
    fn double_humanize_of_long_address_has_no_effect() {
        let bech = Bech::new_b32("juno");
        let canonical1 = JUNO_ADDRESS.as_bytes();
        let humanized1 = bech.humanize(canonical1).unwrap();
        let canonical2 = humanized1.as_bytes();
        let humanized2 = bech.humanize(canonical2).unwrap();
        assert_eq!(humanized1, humanized2);
    }

    #[test]
    fn canonicalize_and_humanize_restores_original() {
        let bech = Bech::new_b32("juno");

        // simple
        let original = String::from("shorty");
        let canonical = bech.canonicalize(&original).unwrap();
        let recovered = bech.humanize(&canonical).unwrap();
        assert_eq!(recovered, original);
        let canonical = bech.canonicalize(recovered.as_ref()).unwrap();
        let recovered = bech.humanize(&canonical).unwrap();
        assert_eq!(recovered, original);

        // normalizes input
        let original = String::from("CosmWasmChef");
        let canonical = bech.canonicalize(&original).unwrap();
        let recovered = bech.humanize(&canonical).unwrap();
        assert_eq!(recovered, "cosmwasmchef");
        let canonical = bech.canonicalize(recovered.as_ref()).unwrap();
        let recovered = bech.humanize(&canonical).unwrap();
        assert_eq!(recovered, "cosmwasmchef");

        // long input (Juno contract address)
        let original = JUNO_ADDRESS.to_string();
        let canonical = bech.canonicalize(&original).unwrap();
        let recovered = bech.humanize(&canonical).unwrap();
        assert_eq!(recovered, original);
        let canonical = bech.canonicalize(recovered.as_ref()).unwrap();
        let recovered = bech.humanize(&canonical).unwrap();
        assert_eq!(recovered, original);
    }

    #[test]
    fn canonicalize_and_humanize_can_be_tricked() {
        let bech = Bech::default();

        // create an address with reversed prefix
        let inner_addr = encode(
            &bech.rev_prefix(),
            b"foobar123".to_base32(),
            Variant::Bech32,
        )
        .unwrap();

        // wrap it again with normal prefix
        let outer_addr = encode(
            &bech.prefix,
            inner_addr.as_bytes().to_base32(),
            Variant::Bech32,
        )
        .unwrap();

        // now canonicalize and humanize again
        let result = bech
            .humanize(&bech.canonicalize(&outer_addr).unwrap())
            .unwrap();

        // the result is different from the input (outer_addr)
        assert_eq!(result, outer_addr);
    }

    #[test]
    fn canonicalize_and_humanize_can_handle_different_variants() {
        let bech = Bech::default();

        // create bech32 address with different variant
        let inner_addr = encode(
            &bech.rev_prefix(),
            b"foobar123".to_base32(),
            Variant::Bech32m,
        )
        .unwrap();

        // nest it as data inside an address with correct variant
        let outer_addr = encode(
            &bech.prefix,
            inner_addr.as_bytes().to_base32(),
            Variant::Bech32,
        )
        .unwrap();

        // now canonicalize and humanize again
        let result = bech
            .humanize(&bech.canonicalize(&outer_addr).unwrap())
            .unwrap();

        assert_eq!(result, outer_addr);
    }
}
