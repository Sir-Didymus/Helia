//! This module contains various error definitions for all things related to database operations.

/// Encodes an error that happened at the persistence layer.
pub enum StorageError {
    ConnectionError,
    InsertFailed
}
