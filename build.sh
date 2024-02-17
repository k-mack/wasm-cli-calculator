#!/usr/bin/env sh

# Build components
cd adder && cargo component build --release && cd -
cd calculator && cargo component build --release && cd -
cd app && cargo component build --release && cd -

# Compose components
wasm-tools compose -d adder/target/wasm32-wasi/release/adder.wasm -o calculator-composed.wasm calculator/target/wasm32-wasi/release/calculator.wasm
wasm-tools compose -d calculator-composed.wasm -o app-composed.wasm app/target/wasm32-wasi/release/app.wasm
