#![allow(unused)]
use bitcoin::address::Address;
use bitcoin::base58;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::Network;
use bitcoin::PrivateKey;
use bip38::Decrypt;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use hex::{decode, FromHex};
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    pub hex_key: String,
    pub pub_key: String,
    #[clap(short, long)]
    pub password: Option<String>,
}

enum Flow {
    Legacy,
    New,
}

fn main() {
    let args = Cli::parse();

    // Prompt menu before deciding
    let flow = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose which flow to run")
        .default(0)
        .item("Legacy mode")
        .item("New mode (WIP)")
        .interact()
        .unwrap();

    match flow {
        0 => run_legacy(args),
        1 => fuzzy_recovery(args),
        _ => unreachable!(),
    }
}

fn fuzzy_recovery(args: Cli) {
    println!("🔍 Running Fuzzy Recovery Mode");

    let password = args.password.unwrap_or_default();
    let variants = generate_fuzzy_variants(&args.hex_key);
    println!("🔠 Generated {} fuzzy variants", variants.len());

    let found = variants
        .into_par_iter()
        .find_any(|variant| match decrypt_bip38_and_get_address(variant, &password) {
            Ok(addr) => {
                if addr == args.pub_key {
                    println!("✅ Found match! Recovered key: {}", variant);
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        });

    if found.is_none() {
        println!("❌ No matching key found.");
    }
}

fn decrypt_bip38_and_get_address(key: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let secp = Secp256k1::new();

    // Decrypt the key using the Decrypt trait
    let secret_key = key.decrypt(password).map_err(|e| format!("BIP38 decryption error: {}", e))?;
    let private_key = PrivateKey::new(secret_key, Network::Bitcoin);
    let address = Address::p2pkh(private_key.public_key(&secp), Network::Bitcoin);
    Ok(address.to_string())
}

fn generate_fuzzy_variants(input: &str) -> Vec<String> {
    let replacements: HashMap<char, Vec<char>> = [
        ('1', vec!['1', 'l', 'I']),
        ('l', vec!['l', '1', 'I']),
        ('I', vec!['I', '1', 'l']),
        ('0', vec!['0', 'O']),
        ('O', vec!['O', '0']),
        ('B', vec!['B', '8']),
        ('8', vec!['8', 'B']),
        ('S', vec!['S', '5']),
        ('5', vec!['5', 'S']),
        ('v', vec!['v', 'V']),
        ('V', vec!['V', 'v']),
        ('2', vec!['2', 'Z']),
        ('Z', vec!['Z', '2']),
        ('6', vec!['6', 'G']),
        ('G', vec!['G', '6']),
        ('X', vec!['X', 'x']),
        ('x', vec!['x', 'X']),
        ('C', vec!['C', 'c']),
        ('c', vec!['c', 'C']),
        ('n', vec!['n', 'm']),
        ('m', vec!['m', 'n']),
        ('Y', vec!['Y', 'V']),
    ]
    .into_iter()
    .collect();

    let sets: Vec<Vec<char>> = input
        .chars()
        .map(|ch| replacements.get(&ch).cloned().unwrap_or_else(|| vec![ch]))
        .collect();

    let mut results = vec![];
    generate_recursive_combinations(&sets, 0, String::new(), &mut results);
    results
}

fn generate_recursive_combinations(
    sets: &[Vec<char>],
    index: usize,
    current: String,
    results: &mut Vec<String>,
) {
    if index == sets.len() {
        results.push(current);
        return;
    }

    for &ch in &sets[index] {
        let mut next = current.clone();
        next.push(ch);
        generate_recursive_combinations(sets, index + 1, next, results);
    }
}

fn base58_to_address(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    let private_key = PrivateKey::from_wif(key)?;
    let public_key = private_key.public_key(&secp);
    let address = Address::p2pkh(public_key, Network::Bitcoin);
    Ok(address.to_string())
}

fn run_legacy(args: Cli) {
    let now = Instant::now();
    let hex_str = args.hex_key.replace('\'', "");
    let pub_key = args.pub_key.replace('\'', "");
    let base58 = is_base58(&hex_str).unwrap();
    let hex_chars = get_chars(base58);
    println!("Generating all possible combinations...");
    let combinations = generate_combinations(&hex_str, &hex_chars);
    let len = combinations.len();
    println!("Generated {len} combinations");
    println!("Checking all possible combinations...");
    combinations
        .into_par_iter()
        .filter(|(c)| check_private_key(base58, c, &pub_key))
        .find_first(|c| {
            println!("Found Private key: {c}");
            true
        });

    let duration = now.elapsed();
    println!("Time elapsed to check all possible keys till the key was found is: {duration:?}");
}

fn check_private_key(base58: bool, c: &str, pub_key: &str) -> bool {
    let mut p2pkh = "".to_string();
    let mut result = false;
    if base58 {
        p2pkh = base58_private_key_to_p2pkh(c).unwrap_or("Error converting to p2pkh".to_string());
    } else {
        p2pkh = hex_private_key_to_p2pkh(c).unwrap_or("Error converting to p2pkh".to_string());
    }

    if p2pkh == pub_key {
        result = true;
    }
    result
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
    let private_key_bytes = match hex::decode(private_key_hex) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid private key hexadecimal format"),
    };

    let secret_key = match SecretKey::from_slice(&private_key_bytes) {
        Ok(key) => key,
        Err(_) => return Err("Invalid private key"),
    };

    let private_key = PrivateKey::new(secret_key, Network::Bitcoin);
    Ok(Address::p2pkh(private_key.public_key(&secp), Network::Bitcoin).to_string())
}

fn base58_private_key_to_p2pkh(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    let private_key = PrivateKey::from_wif(key)?;
    Ok(Address::p2pkh(private_key.public_key(&secp), Network::Bitcoin).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn check_cli_without_arguments_fails() {
        let mut cmd = Command::cargo_bin("btc_keytester").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn combinations_no_underscore() {
        let combinations = generate_combinations("ab", &["1", "2"]);
        assert_eq!(combinations, vec!["ab"]);
    }

    #[test]
    fn test_combinations_one_underscore() {
        let combinations = generate_combinations("a_b", &["1", "2"]);
        assert_eq!(combinations, vec!["a1b", "a2b"]);
    }

    #[test]
    fn test_combinations_two_underscore() {
        let combinations = generate_combinations("a_b_", &["1", "2"]);
        assert_eq!(combinations, vec!["a1b1", "a1b2", "a2b1", "a2b2"]);
    }

    #[test]
    fn combinations_three_underscore() {
        let combinations = generate_combinations("a_b_c", &["1", "2"]);
        assert_eq!(combinations, vec!["a1b1c", "a1b2c", "a2b1c", "a2b2c"]);
    }

    #[test]
    fn hex_private_key_to_p2pkh_error_invalid_format() {
        assert!(hex_private_key_to_p2pkh("c0ffee").is_err());
    }

    #[test]
    fn hex_private_key_to_p2pkh_error_invalid_key() {
        assert!(hex_private_key_to_p2pkh(
            "dc7546c9cef4e980cx63a4cb42efede82c40c0e5fce55c4a7304f32747e029e1"
        )
        .is_err());
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
        assert!(
            is_base58("dc7546c9cef4e980c563a4cb42efede82c40c0ee5fce55_4a7304f32747e029e1").is_err()
        );
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
        assert!(is_base58("KxFC1jmwwCoACiCAWZ3eXa96mBM6tb3TYzGf_f6YwgdGWZgawvrtJ").is_err());
    }

    #[test]
    fn base58_private_key_to_p2pkh_with_private_key() {
        let private_key = "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73sVHnoWn";
        let expected_address = "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH";
        assert_eq!(
            base58_private_key_to_p2pkh(private_key).unwrap_or("".to_string()),
            expected_address
        );
    }

    #[test]
    fn check_private_key_hex_key_with_address_returns_true() {
        let private_key = "dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e0257e";
        let expected_address = "12AbcUTdx39ykDUe4CxwAn65dZ2QSFDEpo";
        let result = check_private_key(false, private_key, expected_address);
        assert!(result);
    }

    #[test]
    fn check_private_key_hex_key_with_wrong_address_returns_false() {
        let private_key = "dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e0257e";
        let expected_address = "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH";
        let result = check_private_key(false, private_key, expected_address);
        assert!(!result);
    }

    #[test]
    fn check_private_key_invalid_hex_key_with_address_returns_false() {
        let private_key = "dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e0257ea";
        let expected_address = "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH";
        let result = check_private_key(false, private_key, expected_address);
        assert!(!result);
    }

    #[test]
    fn check_private_key_base58_key_with_address_returns_true() {
        let private_key = "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73sVHnoWn";
        let expected_address = "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH";
        let result = check_private_key(true, private_key, expected_address);
        assert!(result);
    }

    #[test]
    fn check_private_key_base58_key_with_address_returns_false() {
        let private_key = "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73sVHnoWn";
        let expected_address = "12AbcUTdx39ykDUe4CxwAn65dZ2QSFDEpo";
        let result = check_private_key(true, private_key, expected_address);
        assert!(!result);
    }

    #[test]
    fn check_private_key_invalid_base58_key_with_address_returns_false() {
        let private_key = "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi33qYjgd9M7rFU73sVHnoWn";
        let expected_address = "12AbcUTdx39ykDUe4CxwAn65dZ2QSFDEpo";
        let result = check_private_key(true, private_key, expected_address);
        assert!(!result);
    }
}
