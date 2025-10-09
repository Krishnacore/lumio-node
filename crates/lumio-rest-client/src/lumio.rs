// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use lumio_api_types::U64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LumioCoin {
    pub value: U64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub coin: LumioCoin,
}

impl Balance {
    pub fn get(&self) -> u64 {
        *self.coin.value.inner()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LumioVersion {
    pub major: U64,
}
