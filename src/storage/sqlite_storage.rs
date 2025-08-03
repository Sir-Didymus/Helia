//! This module contains the SQLite implementation of the [Storage] trait.

use rusqlite::Connection;

use crate::{
    model::action::Action,
    storage::{
        Storage,
        migration::{MIGRATIONS, Migration, MigrationSource},
        storage_error::StorageError,
    },
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
    fn schema_version(&self) -> Result<u32, StorageError> {
        let sql = "PRAGMA user_version;";
        let params = [];
        let mapper = |row: &rusqlite::Row| row.get::<_, i32>(0);

        let result = self.conn.query_row(sql, params, mapper);

        match result {
            Ok(version) => Ok(version as u32),
            Err(_error) => Err(StorageError::QueryFailed),
        }
    }

    fn run_migrations(&mut self, source: &dyn MigrationSource) -> Result<u32, StorageError> {
        let current_version = self.schema_version()?;

        let pending_migrations: Vec<&Migration> = source
            .migrations()
            .iter()
            .filter(|m| m.version > current_version)
            .collect();

        if pending_migrations.is_empty() {
            // Nothing to do, return the current version
            return Ok(current_version);
        }

        let tx = self
            .conn
            .transaction()
            .map_err(|_| StorageError::TransactionInitFailed)?;

        for migration in &pending_migrations {
            tx.execute_batch(migration.sql).map_err(|err| {
                eprint!(
                    "Migration {} failed during execution:\nSQL:\n{}\nError:\n{}",
                    migration.version, migration.sql, err
                );
                StorageError::MigrationFailed(migration.version)
            })?;

            // `PRAGMA user_version` only supports literal values, no placeholders.
            let pragma_sql = format!("PRAGMA user_version = {}", migration.version);

            tx.execute_batch(&pragma_sql).map_err(|err| {
                eprint!(
                    "Failed to set PRAGMA user_version to {}:\n{}",
                    migration.version, err
                );
                StorageError::MigrationFailed(migration.version)
            })?;
        }

        let result = tx.commit();
        if let Err(error) = result {
            eprint!("Failed to commit transaction:{error}");
            return Err(StorageError::TransactionCommitFailed);
        }

        // Safe: cannot be `None`, see the `ìs_empty` check above
        Ok(pending_migrations.last().unwrap().version)
    }

    fn insert_action(&self, action: &Action) -> Result<(), StorageError> {
        Err(StorageError::InsertFailed)
    }
}
