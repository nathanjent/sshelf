# Shh Elf

Will silence be golden for this elf?

Trying out the [WASM-4](https://wasm4.org/) fantasy console.

## Develop

## Requirements

- [rust toolchain](https://www.rust-lang.org/tools/install)
- [wasm4](https://wasm4.org/docs/getting-started/setup).

### Release build

Use make to convert assets to sprite data and compile.

    make

### Run locally in browser

    w4 run target/wasm32-unknown-unknown/release/pong_w4.wasm
