[![Release](https://github.com/ragi96/BTCKeyTester/actions/workflows/release.yml/badge.svg)](https://github.com/ragi96/BTCKeyTester/actions/workflows/release.yml) [![Build](https://github.com/ragi96/BTCKeyTester/actions/workflows/build.yml/badge.svg)](https://github.com/ragi96/BTCKeyTester/actions/workflows/build.yml) [![Quality Gate Status](https://sonarqube.ragilab.science/api/project_badges/measure?project=ragi96_BTCKeyTester_AYYcWiGLfuhb9MxETE2T&metric=alert_status&token=sqb_bdcf67015e73bdb5bae6955d83a36a7d51ecb5c0)](https://sonarqube.ragilab.science/dashboard?id=ragi96_BTCKeyTester_AYYcWiGLfuhb9MxETE2T) [![codecov](https://codecov.io/github/ragi96/BTCKeyTester/branch/main/graph/badge.svg?token=LHFO3RR2Q8)](https://codecov.io/github/ragi96/BTCKeyTester)

# BTCKeyTester

If you have written down the private key of your Bitcoin address in HEX or Base58 but can't read it completely, this is a potential solution.

## Usage

Replace all chars you can't read with a \_ (e.g. dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029\_\_)
It will generate all possible combinations of the private key and check if the associated address is the same as the one you have written down.

### General

`btc_keytester <private_key> <address>`

### Linux

Hex:

`./btc_keytester dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029__ 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

Base58:

`./btc_keytester KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73s_HnoWn 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH`

### Windows

Hex:

`.\btc_keytester.exe dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029__ 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

Base58:

`.\btc_keytester.exe KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73s_HnoWn 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH`

### MAC OS

Hex:

`btc_keytester dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029__ 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

Base58:

`btc_keytester KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73s_HnoWn 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH`

## Development

### cargo run

Hex:

`cargo run -- dc7546c9cef4e980c563a4cb42efede82c40c0e5fce55c4a7304f32747e029__ 1JwvWezRrU2yDh1eSwWezyrx3SyKYmtFDQ`

Base58:

`cargo run -- KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73s_HnoWn 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH`

### cargo build

`cargo build --release`

### cargo test

`cargo test`

### cargo fmt

`cargo fmt`

### cargo clippy

`cargo clippy --all --all-targets --all-features -- -D warnings`
