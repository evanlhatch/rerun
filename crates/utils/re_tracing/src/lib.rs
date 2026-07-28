//! Shim: forwards Rerun profiling macros to flatland_observe.
//! Uses $crate:: so downstream crates don't need flatland_observe dependency.

// Re-export flatland_observe macros so $crate::function_scope!() resolves.
pub use flatland_observe::{function_scope, scope};

/// Create a profile scope based on the function name.
#[macro_export]
macro_rules! profile_function {
    () => {
        $crate::function_scope!();
    };
    ($tag:expr) => {
        $crate::function_scope!($tag);
    };
}

/// Create a profiling scope with a custom name.
#[macro_export]
macro_rules! profile_scope {
    ($name:expr) => {
        $crate::scope!($name);
    };
    ($name:expr, $tag:expr) => {
        $crate::scope!($name, $tag);
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

/// Profile function if condition holds.
#[macro_export]
macro_rules! profile_function_if {
    ($cond:expr) => {
        if $cond {
            $crate::function_scope!();
        }
    };
    ($cond:expr, $tag:expr) => {
        if $cond {
            $crate::function_scope!($tag);
        }
    };
}

/// Profile scope if condition holds.
#[macro_export]
macro_rules! profile_scope_if {
    ($cond:expr, $name:expr) => {
        if $cond {
            $crate::scope!($name);
        }
    };
    ($cond:expr, $name:expr, $tag:expr) => {
        if $cond {
            $crate::scope!($name, $tag);
        }
    };
}
