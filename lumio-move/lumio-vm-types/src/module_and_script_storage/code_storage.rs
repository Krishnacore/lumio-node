// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::module_and_script_storage::module_storage::LumioModuleStorage;
use move_binary_format::file_format::CompiledScript;
use move_vm_runtime::Script;
use move_vm_types::code::ScriptCache;

/// Represents code storage used by the Lumio blockchain, capable of caching scripts and modules.
pub trait LumioCodeStorage:
    LumioModuleStorage + ScriptCache<Key = [u8; 32], Deserialized = CompiledScript, Verified = Script>
{
}

impl<T> LumioCodeStorage for T where
    T: LumioModuleStorage
        + ScriptCache<Key = [u8; 32], Deserialized = CompiledScript, Verified = Script>
{
}
