CLI calculator application built using Wasm components.

To run,

```shell
# Build and compose components
./build.sh

# Run WASI-CLI application using wasmtime
wasmtime run --wasm component-model wasi-cli-app.wasm 1 2 add

# Build and run the native CLI application that has wasmtime embedded into it
cli-app/target/release/cli-app 1 2 add
```

It's recommended to use Wasmtime 17+.
