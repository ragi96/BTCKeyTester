#![allow(unused)]
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::util::{address::Address, base58};
use bitcoin::PrivateKey;
use clap::Parser;
use hex::{decode, FromHex};
use rayon::prelude::*;
use std::error::Error;

#[derive(Parser)]
struct Cli {
    hex_key: String,
    pub_key: String,
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let args = Cli::parse();
    let hex_str = args.hex_key.replace('\'', "");
    let pub_key = args.pub_key.replace('\'', "");
    let base58 = is_base58(&hex_str).unwrap();
    let hex_chars = get_chars(base58);
    let combinations = generate_combinations(&hex_str, &hex_chars);

    combinations.into_par_iter().for_each(|(c)| {
        let mut p2pkh = "".to_string();
        if (base58) {
            p2pkh =
                base58_private_key_to_p2pkh(&c).unwrap_or("Error converting to p2pkh".to_string());
        } else {
            p2pkh = hex_private_key_to_p2pkh(&c).unwrap_or("Error converting to p2pkh".to_string());
        }
        if p2pkh == pub_key {
            println!("Found private key: {c}");
        }
    });

    let duration = now.elapsed();
    println!("Time elapsed to check all possible keys is: {duration:?}");
}

fn is_base58(key: &str) -> Result<bool, Box<dyn Error>> {
    if key.len() == 64 {
        return Ok(false);
    }

    if key.len() == 52 {
        return Ok(true);
    }
    Err(From::from(
        "Key is not a valid hexadecimal or base58 representation of a Bitcoin private key",
    ))
}

fn get_chars(base58: bool) -> Vec<&'static str> {
    if (base58) {
        vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H",
            "J", "K", "L", "M", "N", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a",
            "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "m", "n", "o", "p", "q", "r", "s",
            "t", "u", "v", "w", "x", "y", "z",
        ]
    } else {
        vec![
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
        ]
    }
}

fn generate_combinations(hex_str: &str, chars: &[&str]) -> Vec<String> {
    let mut combinations = vec![];

    fn generate_combinations_helper(
        current: String,
        hex_str: &str,
        chars: &[&str],
        combinations: &mut Vec<String>,
    ) {
        if !hex_str.contains('_') {
            combinations.push(current + hex_str);
            return;
        }

        let pos = hex_str.find('_').unwrap();
        let (start, rest) = hex_str.split_at(pos);

        for hex_char in chars {
            generate_combinations_helper(
                current.clone() + start + hex_char,
                &rest[1..],
                chars,
                combinations,
            );
        }
    }

    generate_combinations_helper(String::new(), hex_str, chars, &mut combinations);
    combinations
}

fn hex_private_key_to_p2pkh(private_key_hex: &str) -> Result<String, &'static str> {
    let secp = Secp256k1::new();
    // Convert hexadecimal private key string to bytes
    let private_key_bytes = match hex::decode(private_key_hex) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid private key hexadecimal format"),
    };

    // Create secret key from bytes
    let secret_key = match SecretKey::from_slice(&private_key_bytes) {
        Ok(key) => key,
        Err(_) => return Err("Invalid private key"),
    };

    let private_key = PrivateKey::new(secret_key, Network::Bitcoin);
    Ok(Address::p2pkh(&private_key.public_key(&secp), Network::Bitcoin).to_string())
}

fn base58_private_key_to_p2pkh(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    let private_key = PrivateKey::from_wif(key)?;
    Ok(Address::p2pkh(&private_key.public_key(&secp), Network::Bitcoin).to_string())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::process::Command;
    use std::str;

    #[test]
    fn test_combinations_no_star() {
        let combinations = generate_combinations("ab", &["1", "2"]);
        assert_eq!(combinations, vec!["ab"]);
    }

    #[test]
    fn test_combinations_one_star() {
        let combinations = generate_combinations("a_b", &["1", "2"]);
        assert_eq!(combinations, vec!["a1b", "a2b"]);
    }

    #[test]
    fn test_combinations_two_star() {
        let combinations = generate_combinations("a_b_", &["1", "2"]);
        assert_eq!(combinations, vec!["a1b1", "a1b2", "a2b1", "a2b2"]);
    }

    #[test]
    fn combinations_three_star() {
        let combinations = generate_combinations("a_b_c", &["1", "2"]);
        assert_eq!(combinations, vec!["a1b1c", "a1b2c", "a2b1c", "a2b2c"]);
    }

    #[test]
    fn hex_private_key_to_p2pkh_error_invalid_format() {
        assert!(matches!(hex_private_key_to_p2pkh("c0ffee"), Err(_)));
    }

    #[test]
    fn hex_private_key_to_p2pkh_error_invalid_key() {
        assert!(matches!(
            hex_private_key_to_p2pkh(
                "dc7546c9cef4e980cx63a4cb42efede82c40c0e5fce55c4a7304f32747e029e1"
            ),
            Err(_)
        ));
    }

    #[test]
    fn hex_private_key_to_p2pkh_success() {
        let p2pkh = hex_private_key_to_p2pkh(
            "dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029e1",
        )
        .unwrap();
        assert_eq!("1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ", p2pkh)
    }

    #[test]
    fn get_chars_base58() {
        let chars = get_chars(true);
        assert_eq!(chars.len(), 58);
    }

    #[test]
    fn get_chars_hex() {
        let chars = get_chars(false);
        assert_eq!(chars.len(), 16);
    }

    #[test]
    fn is_base58_hex_key() {
        let base58 =
            is_base58("dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029e1").unwrap();
        assert!(!base58);
    }

    #[test]
    fn is_base58_hex_key_with_underscore() {
        let base58 =
            is_base58("dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55_4a7304f32747e029e1").unwrap();
        assert!(!base58);
    }

    #[test]
    fn is_base58_hex_key_with_underscore_error() {
        assert!(matches!(
            is_base58("dc7546c9cef4e980c563a4cb42efede82c40c0ee5fce55_4a7304f32747e029e1"),
            Err(_)
        ));
    }

    #[test]
    fn is_base58_base58_key() {
        let base58 = is_base58("KxFC1jmwwCoACiCAWZ3eXa96mBM6tb3TYzGmf6YwgdGWZgawvrtJ").unwrap();
        assert!(base58);
    }

    #[test]
    fn is_base58_base58_key_with_underscore() {
        let base58 = is_base58("KxFC1jmwwCoACiCAWZ3eXa96mBM6tb3TYzG_f6YwgdGWZgawvrtJ").unwrap();
        assert!(base58);
    }

    #[test]
    fn is_base58_base58_key_with_underscore_error() {
        assert!(matches!(
            is_base58("KxFC1jmwwCoACiCAWZ3eXa96mBM6tb3TYzGf_f6YwgdGWZgawvrtJ"),
            Err(_)
        ));
    }

    #[test]
    fn test_base58_private_key_to_p2pkh_with_private_key() {
        let private_key = "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73sVHnoWn";
        let expected_address = "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH";
        assert_eq!(
            base58_private_key_to_p2pkh(private_key).unwrap_or("".to_string()),
            expected_address
        );
    }
}
