#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=cli

echo "Building tools and services docker images"
echo "PROFILE: $PROFILE"
echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p lumio \
    -p lumio-backup-cli \
    -p lumio-faucet-service \
    -p lumio-fn-check-client \
    -p lumio-node-checker \
    -p lumio-openapi-spec-generator \
    -p lumio-telemetry-service \
    -p lumio-keyless-pepper-service \
    -p lumio-debugger \
    -p lumio-transaction-emitter \
    -p lumio-api-tester \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    lumio
    lumio-faucet-service
    lumio-node-checker
    lumio-openapi-spec-generator
    lumio-telemetry-service
    lumio-keyless-pepper-service
    lumio-fn-check-client
    lumio-debugger
    lumio-transaction-emitter
    lumio-api-tester
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done

# Build the Lumio Move framework and place it in dist. It can be found afterwards in the current directory.
echo "Building the Lumio Move framework..."
(cd dist && cargo run --locked --profile=$PROFILE --package lumio-framework -- release)
