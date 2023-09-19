/*

use bech32::{decode, encode, FromBase32, ToBase32, Variant};

/// Default bech32 prefix.
pub const BECH32_PREFIX: &str = "cosmwasm";

const B32: Variant = Variant::Bech32;

pub fn b32_canonicalize(b32_prefix: &'static str, input: &str) -> Option<Vec<u8>> {
    if let Ok((prefix, data, variant)) = decode(input) {
        if prefix == b32_prefix && variant == B32 {
            if let Ok(bytes) = Vec::<u8>::from_base32(&data) {
                return Some(bytes);
            }
        }
    } else if let Ok(data) = encode(
        &b32_internal_prefix(b32_prefix),
        input.to_lowercase().as_bytes().to_base32(),
        Variant::Bech32,
    ) {
        return Some(data.as_bytes().to_vec());
    }
    None
}

pub fn b32_humanize(b32_prefix: &'static str, canonical: &[u8]) -> Option<String> {
    if let Ok((prefix, data, variant)) = decode(&String::from_utf8_lossy(canonical)) {
        if prefix == b32_internal_prefix(b32_prefix) && variant == B32 {
            if let Ok(bytes) = Vec::<u8>::from_base32(&data) {
                return Some(String::from_utf8_lossy(&bytes).to_string());
            }
        }
    } else if let Ok(data) = encode(b32_prefix, canonical.to_base32(), B32) {
        return Some(data);
    }
    None
}

fn b32_internal_prefix(prefix: &'static str) -> String {
    prefix.chars().rev().collect::<String>()
}



 */
