//! The 'storage' module specifies and implements the database API.
//!
//! This module is not meant to be called from outside the Helia GTD API, but is only used as in
//! internal abstraction layer. It separates the core GTD API from database concerns.
//! The storage module itself has no knowledge of GTD specific workflows, and thus only offers
//! basic **CRUD** operations.

use crate::{model::action::Action, storage::{sqlite_storage::SqliteStorage, storage_error::StorageError}};

// Error definitions for everything regarding persistence.
mod storage_error;
// SQLite implementation
mod sqlite_storage;

/// Behavior of the storage layer.
pub trait Storage {
    /// Tries to insert an action.
    fn insert_action(&self, action: &Action) -> Result<(), StorageError>;
}

/// Factory method returning a new production ready [Storage] instance.
pub fn new_production_storage(db_path: &str) -> Result<impl Storage, StorageError> {
    SqliteStorage::new_persistence(db_path)
}

