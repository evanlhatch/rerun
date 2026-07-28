//! Shim: forwards Rerun logging macros to tracing.

pub mod debug_assert;

pub use tracing::{debug, error, info, trace, warn, Level};
pub use log_once::{debug_once, error_once, info_once, trace_once, warn_once};

/// Initialize logging.
/// In flatland, this should be called via flatland_observe::init() instead.
/// Kept here for API compatibility.
/// Log a warning once, but only in debug mode.
#[macro_export]
macro_rules! debug_warn_once {
    ($($arg:tt)+) => {
        #[cfg(debug_assertions)]
        $crate::warn_once!($($arg)+);
    };
}

/// Returns whether the current build is in "very strict" mode.
/// In our vendored build, we always return false.
pub fn is_rerun_very_strict() -> bool {
    false
}

pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    // flatland_observe subscriber would be installed by the app.
    // This is a no-op for vendored crates that don't need it.
    Ok(())
}

/// Extension trait adding context to Result types (used by vendored Rerun code).
pub trait ResultExt<T, E> {
    fn with_context<C, F>(self, f: F) -> Result<T, anyhow::Error>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E: std::fmt::Display + Send + Sync + 'static> ResultExt<T, E> for Result<T, E> {
    fn with_context<C, F>(self, f: F) -> Result<T, anyhow::Error>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| anyhow::anyhow!("{}", f()).context(e))
    }
}
