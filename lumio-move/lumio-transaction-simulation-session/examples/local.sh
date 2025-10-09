#!/bin/bash

set -e

rm -rf sess

# initialize session
cargo run -p lumio -- move sim init --path sess

# fund account with 1 APT
cargo run -p lumio -- move sim fund --session sess --account default --amount 100000000

# transfer 100 Octa to self
cargo run -p lumio -- move run --session sess --function-id 0x1::lumio_account::transfer --args address:default u64:100

# view account sequence number
cargo run -p lumio -- move view --session sess --function-id 0x1::account::get_sequence_number --args address:default
