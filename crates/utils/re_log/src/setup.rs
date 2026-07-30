//! Logging setup — no-op in flatland vendor.
//!
//! The real log backend is configured by `flatland_observe::init()`.
//! `re_log` macros (`debug!`, `warn!`, `warn_once!`, etc.) emit through the
//! `log` crate, which `logforth` captures. Calling `setup_logging` here
//! would install a competing `tracing` subscriber and conflict.
//!
//! These functions are kept as no-ops so the 96 call sites in the fork
//! compile without modification.

use std::sync::Once;

static START: Once = Once::new();

/// No-op — log backend configured by `flatland_observe::init()`.
pub fn setup_logging() {
    setup_logging_with_filter("info");
}

/// No-op — log backend configured by `flatland_observe::init()`.
/// Only sets the max log level from the filter string.
pub fn setup_logging_with_filter(log_filter: &str) {
    START.call_once(|| {
        use std::str::FromStr as _;
        let primary_log_filter = log_filter.split(',').next().unwrap_or("info");
        let max_level =
            log::LevelFilter::from_str(primary_log_filter).unwrap_or(log::LevelFilter::Info);
        log::set_max_level(max_level);
    });
}

// ── PanicOnWarnScope — kept as a no-op stub for API compat ────────────────

thread_local! {
    static PANIC_ON_WARN_SCOPE_DEPTH: std::sync::atomic::AtomicIsize = const { std::sync::atomic::AtomicIsize::new(0) };
}

/// Scope for enabling panic on warn — no-op stub in flatland vendor.
pub struct PanicOnWarnScope {
    not_send_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}

impl PanicOnWarnScope {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        PANIC_ON_WARN_SCOPE_DEPTH.with(|enabled| {
            enabled.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });
        Self {
            not_send_sync: Default::default(),
        }
    }
}

impl Drop for PanicOnWarnScope {
    fn drop(&mut self) {
        PANIC_ON_WARN_SCOPE_DEPTH.with(|enabled| {
            enabled.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        });
    }
}

impl Default for PanicOnWarnScope {
    fn default() -> Self {
        Self::new()
    }
}
