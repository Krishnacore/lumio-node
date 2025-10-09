// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

/// Chain ID of the current chain
pub const X_LUMIO_CHAIN_ID: &str = "X-Lumio-Chain-Id";
/// Current epoch of the chain
pub const X_LUMIO_EPOCH: &str = "X-Lumio-Epoch";
/// Current ledger version of the chain
pub const X_LUMIO_LEDGER_VERSION: &str = "X-Lumio-Ledger-Version";
/// Oldest non-pruned ledger version of the chain
pub const X_LUMIO_LEDGER_OLDEST_VERSION: &str = "X-Lumio-Ledger-Oldest-Version";
/// Current block height of the chain
pub const X_LUMIO_BLOCK_HEIGHT: &str = "X-Lumio-Block-Height";
/// Oldest non-pruned block height of the chain
pub const X_LUMIO_OLDEST_BLOCK_HEIGHT: &str = "X-Lumio-Oldest-Block-Height";
/// Current timestamp of the chain
pub const X_LUMIO_LEDGER_TIMESTAMP: &str = "X-Lumio-Ledger-TimestampUsec";
/// Cursor used for pagination.
pub const X_LUMIO_CURSOR: &str = "X-Lumio-Cursor";
/// The cost of the call in terms of gas. Only applicable to calls that result in
/// function execution in the VM, e.g. view functions, txn simulation.
pub const X_LUMIO_GAS_USED: &str = "X-Lumio-Gas-Used";
/// Provided by the client to identify what client it is.
pub const X_LUMIO_CLIENT: &str = "x-lumio-client";
