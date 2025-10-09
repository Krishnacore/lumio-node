// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use lumio_infallible::Mutex;
use lumio_logger::info;
use lumio_storage_interface::state_store::state_view::cached_state_view::CachedStateView;
use lumio_vm::{
    sharded_block_executor::{local_executor_shard::LocalExecutorClient, ShardedBlockExecutor},
    LumioVM,
};
use once_cell::sync::Lazy;
use std::sync::Arc;

pub static SHARDED_BLOCK_EXECUTOR: Lazy<
    Arc<Mutex<ShardedBlockExecutor<CachedStateView, LocalExecutorClient<CachedStateView>>>>,
> = Lazy::new(|| {
    info!("LOCAL_SHARDED_BLOCK_EXECUTOR created");
    Arc::new(Mutex::new(
        LocalExecutorClient::create_local_sharded_block_executor(LumioVM::get_num_shards(), None),
    ))
});
