#![allow(unused)]

use clap::Parser;
use hex::FromHex;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    hex_key: String,
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

fn main() {
    let args = Cli::parse();
    let hex_str = args.hex_key;

    let combinations = generate_combinations(&hex_str);
    for c in combinations {
        println!("{c}");
    }
}
