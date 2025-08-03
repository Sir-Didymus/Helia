//! This module contains various error definitions for all things related to database operations.

use thiserror::Error;

/// Encodes an error that happened at the persistence layer.
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("could not connect to the database")]
    ConnectionError,

    #[error("insert operation failed")]
    InsertFailed,

    #[error("migration to version {0} failed")]
    MigrationFailed(u32),

    #[error("query execution failed")]
    QueryFailed,

    #[error("failed to commit the transaction")]
    TransactionCommitFailed,

    #[error("failed to begin the transaction")]
    TransactionInitFailed,
}
