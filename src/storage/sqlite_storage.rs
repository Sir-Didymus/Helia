//! This module contains the SQLite implementation of the [Storage] trait.

use rusqlite::Connection;
use tracing::{debug, error, info, instrument};

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

    #[instrument(skip(self, source))]
    fn run_migrations(&mut self, source: &dyn MigrationSource) -> Result<u32, StorageError> {
        let current_version = self.schema_version()?;
        debug!(current_version, "Fetched current schema version.");

        let pending_migrations: Vec<&Migration> = source
            .migrations()
            .iter()
            .filter(|m| m.version > current_version)
            .collect();

        if pending_migrations.is_empty() {
            // Nothing to do, return the current version
            info!("No pending migrations. Database is up to date at version {current_version}.");
            return Ok(current_version);
        }

        info!(
            count = pending_migrations.len(),
            "Running pendning migrations."
        );

        let tx = self.conn.transaction().map_err(|err| {
            error!(
                error = %err,
                "Failed to initiate the transaction."
            );
            StorageError::TransactionInitFailed
        })?;

        for migration in &pending_migrations {
            debug!(version = migration.version, "Running migration.");

            if let Err(err) = tx.execute_batch(migration.sql) {
                error!(
                    version = migration.version,
                    sql = migration.sql,
                    error = %err,
                    "Migration failed during execution."
                );
                return Err(StorageError::MigrationFailed(migration.version));
            }

            // `PRAGMA user_version` only supports literal values, no placeholders.
            let pragma_sql = format!("PRAGMA user_version = {}", migration.version);

            if let Err(err) = tx.execute_batch(&pragma_sql) {
                error!(
                        version = migration.version,
                        error = %err,
                        "Failed to set PRAGAMA usser_version."
                );
                return Err(StorageError::MigrationFailed(migration.version));
            }
        }

        if let Err(err) = tx.commit() {
            error!(error = %err, "Failed to commit transaction.");
            return Err(StorageError::TransactionCommitFailed);
        }

        // Safe: cannot be `None`, see the `ìs_empty` check above
        let final_version = pending_migrations.last().unwrap().version;
        info!(final_version, "All migrations applied successfully.");

        Ok(final_version)
    }

    fn insert_action(&self, action: &Action) -> Result<(), StorageError> {
        Err(StorageError::InsertFailed)
    }
}
