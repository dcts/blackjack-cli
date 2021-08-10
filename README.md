# Blackjack CLI Game
Play a casual game of blackjack in the terminal. Written in Rust.

### prequesites
1. you need to have Rust installed ([install instructions](https://www.rust-lang.org/tools/install))
2. to compile the rust code to WASM and link with your frontends javascript code, you need `wasm-pack` ([install instructions](https://rustwasm.github.io/wasm-pack/installer/))

### build
```bash
# clone repository
git clone git@github.com:dcts/blackjack-cli.git

# build binary (the binary will be compiled to `target/debug/blackjack-cli`)
cargo build

# run program
cargo run
```
