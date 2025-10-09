// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::on_chain_config::OnChainConfig;
use serde::{Deserialize, Serialize};

/// Defines the version of Lumio Validator software.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct LumioVersion {
    pub major: u64,
}

impl OnChainConfig for LumioVersion {
    const MODULE_IDENTIFIER: &'static str = "version";
    const TYPE_IDENTIFIER: &'static str = "Version";
}

// NOTE: version number for release 1.2 Lumio
// Items gated by this version number include:
//  - the EntryFunction payload type
pub const LUMIO_VERSION_2: LumioVersion = LumioVersion { major: 2 };

// NOTE: version number for release 1.3 of Lumio
// Items gated by this version number include:
//  - Multi-agent transactions
pub const LUMIO_VERSION_3: LumioVersion = LumioVersion { major: 3 };

// NOTE: version number for release 1.4 of Lumio
// Items gated by this version number include:
//  - Conflict-Resistant Sequence Numbers
pub const LUMIO_VERSION_4: LumioVersion = LumioVersion { major: 4 };

// Maximum current known version
pub const LUMIO_MAX_KNOWN_VERSION: LumioVersion = LUMIO_VERSION_4;
