#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=${PROFILE:-release}

echo "Building indexer and related binaries"
echo "PROFILE: $PROFILE"

echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p lumio-indexer-grpc-cache-worker \
    -p lumio-indexer-grpc-file-store \
    -p lumio-indexer-grpc-data-service \
    -p lumio-nft-metadata-crawler \
    -p lumio-indexer-grpc-file-checker \
    -p lumio-indexer-grpc-data-service-v2 \
    -p lumio-indexer-grpc-manager \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    lumio-indexer-grpc-cache-worker
    lumio-indexer-grpc-file-store
    lumio-indexer-grpc-data-service
    lumio-nft-metadata-crawler
    lumio-indexer-grpc-file-checker
    lumio-indexer-grpc-data-service-v2
    lumio-indexer-grpc-manager
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done
