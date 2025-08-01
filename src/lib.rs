//! # Helia GTD
//!
//! This is the Rust library for the **Helia GTD** App.
//! It specifies and implements its API, which provides a fully functioning GTD implementation.
//! This API is meant to be used in conjunction with some kind of user interface, whether graphical or terminal based.

// Helia API
mod api;
// Database layer
mod storage;
// Domain models
pub mod model;
