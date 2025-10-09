// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module defines error types used by `LumioDB`.
use lumio_types::state_store::errors::StateViewError;
use std::sync::mpsc::RecvError;
use thiserror::Error;

/// This enum defines errors commonly used among `LumioDB` APIs.
#[derive(Clone, Debug, Error)]
pub enum LumioDbError {
    /// A requested item is not found.
    #[error("{0} not found.")]
    NotFound(String),
    /// Requested too many items.
    #[error("Too many items requested: at least {0} requested, max is {1}")]
    TooManyRequested(u64, u64),
    #[error("Missing state root node at version {0}, probably pruned.")]
    MissingRootError(u64),
    /// Other non-classified error.
    #[error("LumioDB Other Error: {0}")]
    Other(String),
    #[error("LumioDB RocksDb Error: {0}")]
    RocksDbIncompleteResult(String),
    #[error("LumioDB RocksDB Error: {0}")]
    OtherRocksDbError(String),
    #[error("LumioDB bcs Error: {0}")]
    BcsError(String),
    #[error("LumioDB IO Error: {0}")]
    IoError(String),
    #[error("LumioDB Recv Error: {0}")]
    RecvError(String),
    #[error("LumioDB ParseInt Error: {0}")]
    ParseIntError(String),
}

impl From<anyhow::Error> for LumioDbError {
    fn from(error: anyhow::Error) -> Self {
        Self::Other(format!("{}", error))
    }
}

impl From<bcs::Error> for LumioDbError {
    fn from(error: bcs::Error) -> Self {
        Self::BcsError(format!("{}", error))
    }
}

impl From<RecvError> for LumioDbError {
    fn from(error: RecvError) -> Self {
        Self::RecvError(format!("{}", error))
    }
}

impl From<std::io::Error> for LumioDbError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(format!("{}", error))
    }
}

impl From<std::num::ParseIntError> for LumioDbError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::Other(format!("{}", error))
    }
}

impl From<LumioDbError> for StateViewError {
    fn from(error: LumioDbError) -> Self {
        match error {
            LumioDbError::NotFound(msg) => StateViewError::NotFound(msg),
            LumioDbError::Other(msg) => StateViewError::Other(msg),
            _ => StateViewError::Other(format!("{}", error)),
        }
    }
}

impl From<StateViewError> for LumioDbError {
    fn from(error: StateViewError) -> Self {
        match error {
            StateViewError::NotFound(msg) => LumioDbError::NotFound(msg),
            StateViewError::Other(msg) => LumioDbError::Other(msg),
            StateViewError::BcsError(err) => LumioDbError::BcsError(err.to_string()),
        }
    }
}
