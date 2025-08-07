//! Contains Helia's core API over which her functionality can be accessed.

use std::sync::Once;

use crate::core::helia_prod::HeliaProd;
use crate::{
    core::{helia_error::HeliaError, requests::CreateActionRequest},
    model::action::Action,
    storage::{self},
};

// Contains the `Request` types.
// These are used to issue requests to the API.
pub mod requests;
// Contains the API error definitions.
pub mod helia_error;
// Contains the concrete production implementation of the `HeliaCore` trait.
mod helia_prod;

/// [`HeliaCore`] is the main way though which the Helia backend can be accessed.
///
/// To access the API, fetch a [`HeliaCore`] instance by calling the [new_production()] function:
/// ```
/// use helia_backend::HeliaCore;
/// let helia_core = helia_backend::new_production().unwrap();
/// ```
pub trait HeliaCore {
    /// Retrieves the storage version of the backend.
    fn storage_version(&self) -> Result<u32, HeliaError>;

    /// Runs all pending storage migrations.
    fn run_migrations(&mut self) -> Result<u32, HeliaError>;

    /// Tries to create an [Action].
    fn create_action(&self, request: CreateActionRequest) -> Result<Action, HeliaError>;
}

/// Factory function creating a new [HeliaCore] instance ready for production.
pub fn new_production() -> Result<impl HeliaCore, HeliaError> {
    let storage_result = storage::new_production_storage("helia.db");
    let storage = match storage_result {
        Ok(storage) => storage,
        Err(err) => return Err(HeliaError::StorageConnectionFailed { storage_err: err }),
    };

    Ok(HeliaProd::new(storage))
}

/// Factory function creating a new [HeliaCore] instance for testing purposes (in-memory).
pub fn new_testing() -> Result<impl HeliaCore, HeliaError> {
    let storage_result = storage::new_in_memory_storage();
    let storage = match storage_result {
        Ok(storage) => storage,
        Err(err) => return Err(HeliaError::StorageConnectionFailed { storage_err: err }),
    };

    Ok(HeliaProd::new(storage))
}

/// Initiates logging for the Helia backend.
pub fn init_logging() {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .init();
    });
}

#[cfg(test)]
mod tests {
    use crate::core;

    use super::*;

    #[test]
    fn test_storage_version_for_new_storage_is_0() {
        let helia_core = core::new_testing().unwrap();
        let storage_version = helia_core.storage_version().unwrap();
        assert_eq!(storage_version, 0);
    }
}
