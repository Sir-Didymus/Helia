//! This module contains error definitions for the Helia GTD API.

/// This enum encodes all the possible API errors that can occur during runtime.
pub enum HeliaError {
    // Database Errors
    PersistenceFailed,
    DbConnectionFailed
}

