//! Production implementation of the [`HeliaCore`] trait.

use crate::{
    core::{helia_error::HeliaError, requests::CreateActionRequest, HeliaCore},
    model::action::Action,
    storage::{migration, Storage},
};

/// [`HeliaProd`] is the production implementation of the [`HeliaCore`] trait.
pub struct HeliaProd<S: Storage> {
    storage: S,
}

impl<S: Storage> HeliaProd<S> {
    /// Returns a new [HeliaProd] instance.
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: Storage> HeliaCore for HeliaProd<S> {
    fn storage_version(&self) -> Result<u32, HeliaError> {
        match self.storage.schema_version() {
            Ok(storage_version) => Ok(storage_version),
            Err(err) => Err(HeliaError::FetchingStorageVersionFailed { storage_err: err }),
        }
    }

    fn run_migrations(&mut self) -> Result<u32, HeliaError> {
        match self
            .storage
            .run_migrations(&migration::builtin_migrations())
        {
            Ok(u32) => Ok(u32),
            Err(err) => Err(HeliaError::MigrationsFailed { storage_err: err }),
        }
    }

    fn create_action(&self, request: CreateActionRequest) -> Result<Action, HeliaError> {
        let action = request.into_action();
        let result = self.storage.insert_action(&action);
        match result {
            Ok(_) => Ok(action),
            Err(err) => Err(HeliaError::CreateActionFailed { storage_err: err }),
        }
    }
}
