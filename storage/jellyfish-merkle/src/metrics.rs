// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use lumio_metrics_core::{
    make_thread_local_int_counter, make_thread_local_int_counter_vec, register_int_counter,
    register_int_gauge, IntCounter, IntGauge,
};
use once_cell::sync::Lazy;

pub static LUMIO_JELLYFISH_LEAF_ENCODED_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "lumio_jellyfish_leaf_encoded_bytes",
        "Lumio jellyfish leaf encoded bytes in total"
    )
    .unwrap()
});

pub static LUMIO_JELLYFISH_INTERNAL_ENCODED_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "lumio_jellyfish_internal_encoded_bytes",
        "Lumio jellyfish total internal nodes encoded in bytes"
    )
    .unwrap()
});

pub static LUMIO_JELLYFISH_LEAF_COUNT: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "lumio_jellyfish_leaf_count",
        "Total number of leaves in the latest JMT."
    )
    .unwrap()
});

make_thread_local_int_counter!(
    pub,
    LUMIO_JELLYFISH_LEAF_DELETION_COUNT,
    "lumio_jellyfish_leaf_deletion_count",
    "The number of deletions happened in JMT."
);

make_thread_local_int_counter_vec!(
    pub,
    COUNTER,
    // metric name
    "lumio_jellyfish_counter",
    // metric description
    "Various counters for the JellyfishMerkleTree",
    // metric labels (dimensions)
    &["name"],
);
