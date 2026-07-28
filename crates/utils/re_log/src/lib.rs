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

/// Check environment variable flag ("0"/"false"/"no"/"off" = false, "1"/"true"/"yes"/"on" = true).
pub fn env_var_flag(var_name: &str) -> Option<bool> {
    match std::env::var(var_name)
        .ok()?
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "" => None,
        "0" | "false" | "no" | "off" => Some(false),
        "1" | "true" | "yes" | "on" => Some(true),
        value => {
            crate::warn_once!(
                "Ignoring unrecognized value {value:?} for environment variable {var_name:?} \
                    (expected one of: 1/true/yes/on, 0/false/no/off); falling back to the default."
            );
            None
        }
    }
}

/// Check if an environment variable is set to a truthy value.
pub fn env_var_is_truthy(var_name: &str) -> bool {
    env_var_flag(var_name).unwrap_or(false)
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

    /// Logs an error if the result is an error and returns the result.
    #[track_caller]
    fn ok_or_log_error(self) -> Option<T>;

    /// Logs an error if the result is an error and returns the result, but only once.
    #[track_caller]
    fn ok_or_log_error_once(self) -> Option<T>;
}

impl<T, E: std::fmt::Display + Send + Sync + 'static> ResultExt<T, E> for Result<T, E> {
    fn with_context<C, F>(self, f: F) -> Result<T, anyhow::Error>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| anyhow::anyhow!("{}", f()).context(e))
    }

    #[track_caller]
    fn ok_or_log_error(self) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(err) => {
                let loc = std::panic::Location::caller();
                tracing::error!("{}:{} {err}", loc.file(), loc.line());
                None
            }
        }
    }

    #[track_caller]
    fn ok_or_log_error_once(self) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(err) => {
                let loc = std::panic::Location::caller();
                crate::error_once!("{}:{} {err}", loc.file(), loc.line());
                None
            }
        }
    }
}
