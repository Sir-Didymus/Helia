//! This module contains helper functions to generate test data.

/// Initiates logging for unit and integration tests.
pub fn helia_init_test_logging() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .init();
    });
}
