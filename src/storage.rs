//! The 'storage' module specifies and implements the database API.
//!
//! This module is not meant to be called from outside the Helia API, but is only used as in
//! internal abstraction layer. It separates the core Helia API from database concerns.
//! The storage module itself has no knowledge of GTD specific workflows, and thus only offers
//! basic **CRUD** operations.

use crate::{
    model::action::Action,
    storage::{
        migration::MigrationSource, sqlite_storage::SqliteStorage, storage_error::StorageError,
    },
};

// Error definitions for everything concerning persistence.
pub mod storage_error;
// SQLite implementation
mod sqlite_storage;
// Contains migration logic
pub mod migration;

/// Behavior of the storage layer.
pub trait Storage {
    /// Retrieves the schema version of the database.
    fn schema_version(&self) -> Result<u32, StorageError>;

    /// Runs all pending migrations in one transaction.
    ///
    /// Takes a source of migrations to apply to the database.
    /// Returns the new schema version or a [`StorageError`].
    fn run_migrations(
        &mut self,
        migration_source: &dyn MigrationSource,
    ) -> Result<u32, StorageError>;

    /// Tries to insert an action.
    fn insert_action(&self, action: &Action) -> Result<(), StorageError>;
}

/// Factory method returning a new production ready [Storage] instance.
pub fn new_production_storage(db_path: &str) -> Result<impl Storage, StorageError> {
    SqliteStorage::new_persistence(db_path)
}

/// Factory method returning a new in-memory [Storage] instance.
pub fn new_in_memory_storage() -> Result<impl Storage, StorageError> {
    SqliteStorage::new_in_memory()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::action::test_utils::dummy_action, storage::migration::builtin_migrations};

    #[test]
    fn test_insert_action_is_ok() {
        let storage = new_in_memory_storage().unwrap();
        let action = dummy_action();
        assert!(storage.insert_action(&action).is_ok());
    }

    #[test]
    fn test_schema_version_for_new_databse_is_0() {
        let storage = new_in_memory_storage().unwrap();
        let schema_version = storage.schema_version().unwrap();
        assert_eq!(schema_version, 0);
    }

    #[test]
    fn test_run_production_migrations_for_new_db_is_ok() {
        let mut storage = new_in_memory_storage().unwrap();
        let result = storage.run_migrations(&builtin_migrations());
        assert!(result.is_ok())
    }
}
