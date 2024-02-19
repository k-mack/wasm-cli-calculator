A Wasm component that accepts three arguments via the command-line interface.
The component exports the `wasi:cli/run` interface, allowing it to be run by any `wasi:cli` host (e.g., [wasmtime](https://wasmtime.dev/)).

Build the component with `cargo component build`.
Then, compose the component with the calculator component using `wasm-tools`.
See [build.sh](../build.sh) for details (or just run the script from its directory).

Running the composed component with `wasmtime`, for example, looks like

```shell
$ wasmtime run --wasm component-model wasi-cli-app.wasm 1 2 subtract
1 - 2 = -1
```
