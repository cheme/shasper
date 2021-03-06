# Substrate Shasper

*Note: This is an experimental project. Everything will break, and it may disappear without any notice!*

This is an implementation of [Shasper](https://github.com/ethereum/eth2.0-specs) beacon chain using the [Substrate framework](https://github.com/paritytech/substrate).

## Status

Currently we have an implementation of Shasper state transition validation algorithms. This is then combined with LMD-GHOST consensus engine to provide a simple Substrate node implementation with block authoring.

## Get Started

To build the project, first install [rustup](https://rustup.rs/), [wasm-gc](https://github.com/alexcrichton/wasm-gc):

```bash
rustup update stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo +nightly install --git https://github.com/alexcrichton/wasm-gc
```

Additionally, install `clang` and `llvm`, e.g. via the package manager for your distribution.

Clone the repo and optionally compile the WebAssembly runtime. Note that if you want to connect to a testnet, please skip compiling WebAssembly runtime step.

```bash
git clone https://github.com/paritytech/shasper.git
cd shasper && ./build.sh
```

You can then execute the client:

```bash
cargo run -- --dev
```

The client will run and then start proposing and importing blocks.

## License

Licensed under GPLv3.
