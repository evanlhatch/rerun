//! Shim: forwards Rerun profiling macros to flatland_observe.
//! Uses $crate:: so downstream crates don't need flatland_observe dependency.

// Re-export flatland_observe macros so $crate::function_scope!() resolves.
pub use flatland_observe::{function_scope, scope};

/// Create a profile scope based on the function name.
#[macro_export]
macro_rules! profile_function {
    ($($arg: tt)*) => {
        $crate::function_scope!($($arg)*);
    };
}

/// Create a profiling scope with a custom name.
#[macro_export]
macro_rules! profile_scope {
    ($($arg: tt)*) => {
        $crate::scope!($($arg)*);
    };
}

/// Create a profiling scope that indicates waiting.
#[macro_export]
macro_rules! profile_wait {
    () => {
        $crate::scope!("[WAIT]");
    };
    ($id:expr) => {
        $crate::scope!(concat!("[WAIT] ", $id));
    };
    ($id:expr, $data:expr) => {
        $crate::scope!(concat!("[WAIT] ", $id), $data);
    };
}

/// Profile function if condition holds. Always active in our shim.
#[macro_export]
macro_rules! profile_function_if {
    ($($arg: tt)*) => {
        $crate::function_scope!($($arg)*);
    };
}

/// Profile scope if condition holds.
#[macro_export]
macro_rules! profile_scope_if {
    ($($arg: tt)*) => {
        $crate::scope!($($arg)*);
    };
}
