//! # helia-backend
//!
//! This is the Rust backend library for *Helia* - a free and open-source productivity app.
//!
//! This backend provides everything needed to build a working, fully fledged-out Helia app.
//! It is meant to be used in conjunction with some kind of user interface, whether graphical or terminal based.
//! It's API tries to be as frontend-independent as possible.
//!
//! The main way to access the API is the [HeliaCore] trait.
//! It specifies the helia backend's core capabilities.

// Re-export types and functions relevant to the API.
pub use crate::core::helia_error::*;
pub use crate::core::requests::*;
pub use crate::core::*;

// Helia core API
mod core;
// Database layer
mod storage;
// Domain models
mod model;
// Helper functions for tests
mod test_utils;
