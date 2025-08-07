//! Error definitions for the Helia core.

use crate::storage::storage_error::StorageError;

use thiserror::Error;

/// This enum encodes all the possible errors that can occur during Helia's runtime.
#[derive(Error, Debug)]
pub enum HeliaError {
    #[error("Failed to create action.")]
    CreateActionFailed { storage_err: StorageError },

    #[error("Failed to fetch database schema version.")]
    FetchingStorageVersionFailed { storage_err: StorageError },

    #[error("Failed to run migrations.")]
    MigrationsFailed { storage_err: StorageError },

    #[error("Could not connect to the storage backend.")]
    StorageConnectionFailed { storage_err: StorageError },
}
