//! Various error definitions for all things related to database operations.

use thiserror::Error;

/// Encodes an error that happened at the persistence layer.
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Could not connect to the database.")]
    ConnectionError,

    #[error("Insert operation failed.")]
    InsertFailed,

    #[error("Migration to schema version {version} failed.")]
    MigrationFailed { version: u32 },

    #[error("Failed to prepare sql statement.")]
    PrepareStatementFailed,

    #[error("Query execution failed.")]
    QueryFailed,

    #[error("Failed to commit transaction.")]
    TransactionCommitFailed,

    #[error("Failed to begin transaction.")]
    TransactionInitFailed,
}
