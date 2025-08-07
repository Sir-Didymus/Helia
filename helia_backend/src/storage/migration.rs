//! Types and utilities for managing database schema migrations in Helia.
//!
//! Migrations are used to update the database schema over time.
//! Each migration is associated with a version number and contains the SQL statements necessary to
//! bring the database from the previous version to the next.
//!
//! The central type is [`Migration`], which represents a single migration.
//! These migrations are embedded in the binary at compile time using the [`include_migration!`]
//! macro, which loads SQL files from the `migrations/` directory located at the root of the project.
//!
//! The [`MigrationSource`] trait defines a source from which migrations can be retrieved via its
//! [`MigrationSource::migrations()`] method.
//! There are two migration sources available, one for testing and one for production.
//!
//! For testing, you can use [`test_migrations()`].
//! For production, use [`builtin_migrations()`].

/// Represents a single database migration step.
///
/// Each [`Migration`] consists of:
/// - a `version` number indicating the target version of the database schema after applying the
///   migrations
/// - the embedded SQL statements (`sql`) needed to perform the migration
#[derive(Clone)]
pub struct Migration {
    /// The target schema version.
    pub(crate) version: u32,
    /// The SQL statements needed to perform this migration.
    pub(crate) sql: &'static str,
}

/// A source for [`Migration`]s.
///
/// To get an instance of [`MigrationSource`], use [`builtin_migrations()`] for production and
/// [`test_migrations()`] for testing.
///
/// Use the [`MigrationSource::migrations()`] method to get all available migrations.
pub trait MigrationSource {
    /// Returns all [`Migration`]s.
    fn migrations(&self) -> &Vec<Migration>;
}

/// Returns an [`MigrationSource`] implementation for production purposes.
pub fn builtin_migrations() -> impl MigrationSource {
    BuiltInMigrations {
        migrations: MIGRATIONS.to_vec(),
    }
}

/// Returns an [`MigrationSource`] implementation for testing purposes.
#[cfg(test)]
#[allow(dead_code)]
pub fn test_migrations(migrations: Vec<Migration>) -> impl MigrationSource {
    TestMigrations { migrations }
}

/// Implementation of [`MigrationSource`] for production purposes.
///
/// This implementation retrieves the [`Migration`]s from the top level `/migrations` directory
/// (via the [`MIGRATIONS`] array.
/// These migrations are compiled into the binary, thus the name `BuiltInMigrations`.
struct BuiltInMigrations {
    migrations: Vec<Migration>,
}

impl MigrationSource for BuiltInMigrations {
    fn migrations(&self) -> &Vec<Migration> {
        &self.migrations
    }
}

/// Implementation of [`MigrationSource`] for testing purposes.
///
/// You can pass this implementation of [`MigrationSource`] a vector of [`Migration`]s you want to
/// execute. This is useful for testing scenarios involving faulty migrations.
#[cfg(test)]
struct TestMigrations {
    migrations: Vec<Migration>,
}

#[cfg(test)]
impl MigrationSource for TestMigrations {
    fn migrations(&self) -> &Vec<Migration> {
        &self.migrations
    }
}

/// Includes a SQL migration file from the `migrations/` directory located at the project root.
macro_rules! include_migration {
    ($file:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations/", $file))
    };
}

/// This static array holds all migrations defined in `migrations/` in version order.
/// It is is used by the storage backend to determine which migrations need to be applied
/// based on the current schema version of the database.
pub static MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    sql: include_migration!("001_initial.sql"),
}];
