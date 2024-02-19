CLI calculator application built using Wasm components.

To run,

```shell
./build.sh
wasmtime run --wasm component-model wasi-cli-app.wasm 1 2 add
```

It's recommended to use Wasmtime 17+.
