//! Cross-cutting support: logging setup, error types, build metadata.

pub mod env;
pub mod meta;

use tracing_subscriber::EnvFilter;

/// Initialize structured logging. Honors `RUST_LOG`; defaults to `info`.
pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
