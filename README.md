# BTCKeyTester

If you have written down the private key of your Bitcoin address in HEX format but can't read it completely, this is a potential solution.

## Usage

Replace all chars you can't read with a \* (e.g. dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029\*\*)
It will generate all possible combinations of the private key and check if the associated address is the same as the one you have written down.

### General

`btc_keytester <private_key> <address>`

### Linux

`./btc_keytester dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029** 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

### Windows

`.\btc_keytester.exe dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029** 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

### MAC OS

`btc_keytester 'dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029**' 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

## Development

### cargo run

`cargo run -- dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029** 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

### cargo build

`cargo build --release`

### cargo test

`cargo test`

### cargo fmt

`cargo fmt`

### cargo clippy

`cargo clippy --all --all-targets --all-features -- -D warnings`
