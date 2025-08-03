//! This module contains error definitions for the Helia API.

/// This enum encodes all the possible API errors that can occur during runtime.
#[derive(Debug)]
pub enum HeliaError {
    // Database Errors
    QueryFailed,
    PersistenceFailed,
    StorageConnectionFailed
}

