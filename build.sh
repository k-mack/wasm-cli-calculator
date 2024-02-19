#!/usr/bin/env bash

# Build components
for dir in adder/ substractor/ calculator/ wasi-cli-command/; do
    cd $dir && cargo component build --release && cd -
done

# Compose components
wasm-tools compose \
    -d adder/target/wasm32-wasi/release/adder.wasm \
    -d subtractor/target/wasm32-wasi/release/subtractor.wasm \
    -o calculator-composed.wasm \
    calculator/target/wasm32-wasi/release/calculator.wasm

wasm-tools compose \
    -d calculator-composed.wasm \
    -o wasi-cli-app.wasm \
    wasi-cli-command/target/wasm32-wasi/release/wasi-cli-command.wasm
