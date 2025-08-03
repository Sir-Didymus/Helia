//! This module contains the concrete implementation of the [`HeliaApi`] trait.

use crate::{
    api::{HeliaApi, helia_error::HeliaError, requests::CreateActionRequest},
    model::action::Action,
    storage::Storage,
};

/// The `HeliaImpl` struct is the concrete implementation of the [`HeliaApi`] trait.
pub struct HeliaImpl<S: Storage> {
    storage: S,
}

impl<S: Storage> HeliaImpl<S> {
    /// Returns a new [HeliaImpl] instance.
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: Storage> HeliaApi for HeliaImpl<S> {
    fn storage_version(&self) -> Result<u32, HeliaError> {
        let Ok(storage_version) =  self.storage.schema_version() else {
            return Err(HeliaError::QueryFailed)
        };
        Ok(storage_version)
    }

    fn create_action(&self, request: CreateActionRequest) -> Result<Action, HeliaError> {
        let action = request.into_action();
        let result = self.storage.insert_action(&action);
        match result {
            Ok(_) => Ok(action),
            Err(_) => Err(HeliaError::PersistenceFailed),
        }
    }
}
