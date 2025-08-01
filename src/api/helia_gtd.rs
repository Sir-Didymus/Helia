//! This module contains the concrete implementation of the [`HeliaApi`] trait.

use crate::{
    api::{HeliaApi, helia_error::HeliaError, requests::CreateActionRequest},
    model::action::Action,
    storage::Storage,
};

/// The `HeliaGtd` struct is the concrete implementation of the [`HeliaApi`] trait.
pub struct HeliaGtd<S: Storage> {
    storage: S,
}

impl<S: Storage> HeliaGtd<S> {
    /// Returns a new [HeliaGtd] instance.
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: Storage> HeliaApi for HeliaGtd<S> {
    fn create_action(&self, request: CreateActionRequest) -> Result<Action, HeliaError> {
        let action = request.into_action();
        let result = self.storage.insert_action(&action);
        match result {
            Ok(_) => Ok(action),
            Err(_) => Err(HeliaError::PersistenceFailed),
        }
    }
}
