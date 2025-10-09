---
id: Lumio-framework
title: Lumio Framework
custom_edit_url: https://github.com/lumio-labs/lumio-core/edit/main/Lumio-move/Lumio-framework/README.md
---

## The Lumio Framework

The Lumio Framework defines the standard actions that can be performed on-chain
both by the Lumio VM---through the various prologue/epilogue functions---and by
users of the blockchain---through the allowed set of transactions. This
directory contains different directories that hold the source Move
modules and transaction scripts, along with a framework for generation of
documentation, ABIs, and error information from the Move source
files. See the [Layout](#layout) section for a more detailed overview of the structure.

## Documentation

Each of the main components of the Lumio Framework and contributing guidelines are documented separately. See them by version below:

* *Lumio tokens* - [main](https://github.com/lumio-labs/lumio-core/blob/main/lumio-move/framework/lumio-token/doc/overview.md), [testnet](https://github.com/lumio-labs/lumio-core/blob/testnet/lumio-move/framework/lumio-token/doc/overview.md), [devnet](https://github.com/lumio-labs/lumio-core/blob/devnet/lumio-move/framework/lumio-token/doc/overview.md)
* *Lumio framework* - [main](https://github.com/lumio-labs/lumio-core/blob/main/lumio-move/framework/lumio-framework/doc/overview.md), [testnet](https://github.com/lumio-labs/lumio-core/blob/testnet/lumio-move/framework/lumio-framework/doc/overview.md), [devnet](https://github.com/lumio-labs/lumio-core/blob/devnet/lumio-move/framework/lumio-framework/doc/overview.md)
* *Lumio stdlib* - [main](https://github.com/lumio-labs/lumio-core/blob/main/lumio-move/framework/lumio-stdlib/doc/overview.md), [testnet](https://github.com/lumio-labs/lumio-core/blob/testnet/lumio-move/framework/lumio-stdlib/doc/overview.md), [devnet](https://github.com/lumio-labs/lumio-core/blob/devnet/lumio-move/framework/lumio-stdlib/doc/overview.md)
* *Move stdlib* - [main](https://github.com/lumio-labs/lumio-core/blob/main/lumio-move/framework/move-stdlib/doc/overview.md), [testnet](https://github.com/lumio-labs/lumio-core/blob/testnet/lumio-move/framework/move-stdlib/doc/overview.md), [devnet](https://github.com/lumio-labs/lumio-core/blob/devnet/lumio-move/framework/move-stdlib/doc/overview.md)

Follow our [contributing guidelines](CONTRIBUTING.md) and basic coding standards for the Lumio Framework.

## Compilation and Generation

The documents above were created by the Move documentation generator for Lumio. It is available as part of the Lumio CLI. To see its options, run:
```shell
lumio move document --help
```

The documentation process is also integrated into the framework building process and will be automatically triggered like other derived artifacts, via `cached-packages` or explicit release building.

## Running Move tests

To test our Move code while developing the Lumio Framework, run `cargo test` inside this directory:

```
cargo test
```

(Alternatively, run `cargo test -p lumio-framework` from anywhere.)

To skip the Move prover tests, run:

```
cargo test -- --skip prover
```

To filter and run **all** the tests in specific packages (e.g., `lumio_stdlib`), run:

```
cargo test -- lumio_stdlib --skip prover
```

(See tests in `tests/move_unit_test.rs` to determine which filter to use; e.g., to run the tests in `lumio_framework` you must filter by `move_framework`.)

To **filter by test name or module name** in a specific package (e.g., run the `test_empty_range_proof` in `lumio_stdlib::ristretto255_bulletproofs`), run:

```
TEST_FILTER="test_range_proof" cargo test -- lumio_stdlib --skip prover
```

Or, e.g., run all the Bulletproof tests:
```
TEST_FILTER="bulletproofs" cargo test -- lumio_stdlib --skip prover
```

To show the amount of time and gas used in every test, set env var `REPORT_STATS=1`.
E.g.,
```
REPORT_STATS=1 TEST_FILTER="bulletproofs" cargo test -- lumio_stdlib --skip prover
```

Sometimes, Rust runs out of stack memory in dev build mode.  You can address this by either:
1. Adjusting the stack size

```
export RUST_MIN_STACK=4297152
```

2. Compiling in release mode

```
cargo test --release -- --skip prover
```

## Layout
The overall structure of the Lumio Framework is as follows:

```
├── lumio-framework                                 # Sources, testing and generated documentation for Lumio framework component
├── lumio-token                                 # Sources, testing and generated documentation for Lumio token component
├── lumio-stdlib                                 # Sources, testing and generated documentation for Lumio stdlib component
├── move-stdlib                                 # Sources, testing and generated documentation for Move stdlib component
├── cached-packages                                 # Tooling to generate SDK from move sources.
├── src                                     # Compilation and generation of information from Move source files in the Lumio Framework. Not designed to be used as a Rust library
├── releases                                    # Move release bundles
└── tests
```
