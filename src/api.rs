//! This module contains the API over which Helia's functionality can be accessed.

use crate::{
    api::{helia_error::HeliaError, helia_impl::HeliaImpl, requests::CreateActionRequest},
    model::action::Action,
    storage::{self},
};

use tracing_subscriber;

// Contains the `Request` types.
// These are used to issue requests to the API.
pub mod requests;
// Contains the API error definitions.
mod helia_error;
// Contains the concrete implementation of the `HeliaApi` trait.
mod helia_impl;

/// The HeliaApi provides everything necessary to build a fully functioning GTD app.
///
/// To access the API, fetch a [HeliaApi] by calling the [new_production()] function:
/// ```
/// use helia::api;
/// let helia_api = helia::new_production().unwrap();
/// helia_api.create_action(&dummy_action);
/// ```
pub trait HeliaApi {
    /// Retrieves the storage version of the backend.
    fn storage_version(&self) -> Result<u32, HeliaError>;

    /// Tries to create an [Action].
    fn create_action(&self, request: CreateActionRequest) -> Result<Action, HeliaError>;
}

/// Factory function creating a new [HeliaApi] instance ready for production.
pub fn new_production() -> Result<impl HeliaApi, HeliaError> {
    let storage_result = storage::new_production_storage("helia.db");
    let storage = match storage_result {
        Ok(storage) => storage,
        Err(_error) => return Err(HeliaError::StorageConnectionFailed),
    };

    Ok(HeliaImpl::new(storage))
}

/// Factory function creating a new [HeliaApi] instance for testing purposes (in-memory).
pub fn new_testing() -> Result<impl HeliaApi, HeliaError> {
    let storage_result = storage::new_in_memory_storage();
    let storage = match storage_result {
        Ok(storage) => storage,
        Err(_error) => return Err(HeliaError::StorageConnectionFailed),
    };

    Ok(HeliaImpl::new(storage))
}

/// Initiates logging for the Helia backend.
#[unsafe(no_mangle)]
pub extern "C" fn helia_init_logging() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .init();
    });
}

#[cfg(test)]
mod tests {
    use crate::api;

    use super::*;

    #[test]
    fn test_storage_version_for_new_storage_is_0() {
        let helia_api = api::new_testing().unwrap();
        let storage_version = helia_api.storage_version().unwrap();
        assert_eq!(storage_version, 0);
    }
}
