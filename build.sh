#!/bin/zsh

# build rust
cargo build --target wasm32-unknown-unknown --release

# bind it, with wasm-bindgen CLI (available here https://crates.io/crates/wasm-bindgen-cli).
# be careful, wasm-bindgen CLI & wasm-bindgen versions have to match, you may have to
# update wasm-bindgen to make it work
wasm-bindgen ./target/wasm32-unknown-unknown/release/chiphuit.wasm --out-dir build --no-typescript --no-modules --remove-name-section --remove-producers-section 

# optimize its size with wasm-opt from binaryen https://github.com/WebAssembly/binaryen
wasm-opt -Oz -o build/chiphuit_bg.wasm build/chiphuit_bg.wasm

# launch http serv
basic-http-server build/
