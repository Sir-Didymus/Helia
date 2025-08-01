//! This module contains the API over which Helia GTD's functionality can be accessed.

use crate::{
    api::{helia_error::HeliaError, helia_gtd::HeliaGtd, requests::CreateActionRequest},
    model::action::Action,
    storage::{self},
};

// Contains the `Request` types.
mod requests;
// Contains the API error definitions.
mod helia_error;
// Contains the concrete implementation of the [`HeliaApi`] trait.
mod helia_gtd;

/// The HeliaApi provides everything necessary to build a fully functioning GTD app.
pub trait HeliaApi {
    /// Tries to create an [Action].
    fn create_action(&self, request: CreateActionRequest) -> Result<Action, HeliaError>;
}

/// Factory function creating a new [HeliaApi] instance ready for production.
pub fn new_production() -> Result<impl HeliaApi, HeliaError> {
    let storage_result = storage::new_production_storage("");
    let storage = match storage_result {
        Ok(storage) => storage,
        Err(_error) => return Err(HeliaError::DbConnectionFailed),
    };

    Ok(HeliaGtd::new(storage))
}
