# BTCKeyTester

If you have written down the private key of your Bitcoin address in HEX or Base58 but can't read it completely, this is a potential solution.

## Usage

Replace all chars you can't read with a \_ (e.g. dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029\_\_)
It will generate all possible combinations of the private key and check if the associated address is the same as the one you have written down.

### General

`btc_keytester <private_key> <address>`

### Linux

`./btc_keytester dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029__ 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

`./btc_keytester KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73s_HnoWn 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH`

### Windows

`.\btc_keytester.exe dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029__ 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

`.\btc_keytester.exe KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73s_HnoWn 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH`

### MAC OS

`btc_keytester dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029__ 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

`btc_keytester KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73s_HnoWn 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH`

## Development

### cargo run

`cargo run -- dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029__ 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

`cargo run -- KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73s_HnoWn 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH`

### cargo build

`cargo build --release`

### cargo test

`cargo test`

### cargo fmt

`cargo fmt`

### cargo clippy

`cargo clippy --all --all-targets --all-features -- -D warnings`
