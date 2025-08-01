//! This module contains the SQLite implementation of the [Storage] trait.

use rusqlite::Connection;

use crate::{
    model::action::Action,
    storage::{Storage, sqlite_storage, storage_error::StorageError},
};

/// The SQLite implementation used for production.
pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    /// Creates a new [SqliteStorage] instance with persistence.
    pub fn new_persistence(db_path: &str) -> Result<Self, StorageError> {
        let conn = Connection::open(db_path);
        let conn = match conn {
            Ok(sqtlite_storage) => sqtlite_storage,
            Err(_sqlite_error) => return Result::Err(StorageError::ConnectionError),
        };

        let sqlite_storage = SqliteStorage { conn };

        Ok(sqlite_storage)
    }

    /// Creates a new in-memory [SqliteStorage].
    pub fn new_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory();
        let conn = match conn {
            Ok(sqtlite_storage) => sqtlite_storage,
            Err(_sqlite_error) => return Result::Err(StorageError::ConnectionError),
        };

        let sqlite_storage = SqliteStorage { conn };

        Ok(sqlite_storage)
    }
}

impl Storage for SqliteStorage {
    fn insert_action(&self, action: &Action) -> Result<(), StorageError> {
        Err(StorageError::InsertFailed)
    }
}
