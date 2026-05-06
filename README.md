# ln-channel-tx-properties

![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

This is a collection of tools related to analysing various channels in the
Lightning network.

The [tx-properties](./tx-properties) crate reads a list of transaction IDs and
queries the Esplora HTTP API to evaluate them using the property heuristic.
It also supports general analysis of transactions within a block range.

The [channel-frequency](./channel-frequency) crate reads a list of transaction
IDs and queries the Esplora HTTP API to evaluate them based on the frequency of
channel openings.

Each subproject contains its own usage instructions.

Note that analyses can take a while due to manual sleeps to respect API rate limits.

## Requirements

1. [rustup](https://rustup.rs/)
    - assumes a C linker is already installed, e.g., `sudo apt install
      build-essential` on Ubuntu or `xcode-select --install` on MacOS.

## Usage

1. Compile everything

      ```bash
        cargo build --release --all
      ```
2. Run all unit tests

      ```bash
        cargo test --release --all
      ```
