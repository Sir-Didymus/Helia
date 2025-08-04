//! This module contains helper functions and utilities for unit tests.

/// Initiates logging for unit tests.
pub fn init_test_logging() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
    });
}
