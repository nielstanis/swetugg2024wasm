# Cargo Component Demo - Rust & WASIp2

This demo will show how with WASI Preview 2 we can build and launch components and a side from filesystem also limit other capabilities.

- `data2file` is a demo that will look familiar and will copy file from to destination
- `wasmtime target/wasm32-wasi/debug/data2file.wasm /etc/hosts ./hosts` will fail; we need file access granted from runtime leve.
- `wasmtime --dir . --dir /etc target/wasm32-wasi/debug/data2file.wasm /etc/hosts ./hosts` will work
- `data2filev2` is a project that has additional functionaly and will also be able to get data form url
- `wasmtime --dir . --dir /etc target/wasm32-wasi/debug/data2filev2.wasm https://www.swetugg.se ./swetugg`
- `wasmtime --dir . --dir /etc -S http target/wasm32-wasi/debug/data2filev2.wasm https://www.swetugg.se ./swetugg` will work 200 no data
- `wasmtime --dir . --dir /etc -S http target/wasm32-wasi/debug/data2filev2.wasm https://swetugg.se/gbg-2024 ./swetugg`
