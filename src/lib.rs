//! # Helia
//!
//! This is the Rust library for *Helia* - a free and open-source productivity app.
//! It specifies and implements its API, which provides a fully functioning GTD implementation.
//! This API is meant to be used in conjunction with some kind of user interface, whether graphical or terminal based.

// Helia API
pub mod api;
// Database layer
mod storage;
// Domain models
pub mod model;
// Helper functions for tests
mod test_utils;
