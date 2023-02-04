#![allow(unused)]
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::util::address::Address;
use bitcoin::util::base58;
use bitcoin::util::key::KeyPair;
use bitcoin::PublicKey;
use clap::Parser;
use hex::{decode, FromHex};

#[derive(Parser)]
struct Cli {
    hex_key: String,
    pub_key: String,
}

fn generate_combinations(hex_str: &str) -> Vec<String> {
    let hex_chars = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
    ];
    let mut combinations = vec![];

    fn generate_combinations_helper(
        current: String,
        hex_str: &str,
        hex_chars: &[&str],
        combinations: &mut Vec<String>,
    ) {
        if !hex_str.contains('*') {
            combinations.push(current + hex_str);
            return;
        }

        let pos = hex_str.find('*').unwrap();
        let (start, rest) = hex_str.split_at(pos);

        for hex_char in hex_chars {
            generate_combinations_helper(
                current.clone() + start + hex_char,
                &rest[1..],
                hex_chars,
                combinations,
            );
        }
    }

    generate_combinations_helper(String::new(), hex_str, &hex_chars, &mut combinations);
    combinations
}

fn private_key_to_p2pkh(private_key_hex: &str) -> Result<String, &'static str> {
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

    let private_key = bitcoin::util::key::PrivateKey::new(secret_key, Network::Bitcoin);
    let p2pkh_address =
        Address::p2pkh(&private_key.public_key(&secp), Network::Bitcoin).to_string();
    Ok(p2pkh_address)
}

fn main() {
    let args = Cli::parse();
    let hex_str = args.hex_key;
    let pub_key = args.pub_key;

    let combinations = generate_combinations(&hex_str);

    let pb = indicatif::ProgressBar::new(combinations.len() as u64);
    let length = combinations.len();
    for (mut counter, c) in combinations.into_iter().enumerate() {
        counter += 1;
        let p2pkh = private_key_to_p2pkh(&c).unwrap();
        if p2pkh == pub_key {
            println!("Found private key: {c}");
            break;
        }
        pb.println(format!("[+] finished #{counter} of {length}"));
        pb.inc(1)
    }
}