//! Shim: re_int — integer utility types.

pub mod unsigned_abs;
pub use unsigned_abs::UnsignedAbs;

/// A type-safe wrapper around an i32 representing a quality of service value.
/// Used in Rerun's data pipeline for LOD / quality selection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualityOfService(pub i32);


/// Trait for saturating numeric casts. Used by vendored code.
pub trait SaturatingCast<T> {
    fn saturating_cast(self) -> T;
}

impl SaturatingCast<u64> for i64 {
    fn saturating_cast(self) -> u64 {
        self as u64
    }
}

impl SaturatingCast<u64> for u64 {
    fn saturating_cast(self) -> u64 {
        self
    }
}

impl SaturatingCast<i64> for u64 {
    fn saturating_cast(self) -> i64 {
        self as i64
    }
}

impl SaturatingCast<u32> for i64 {
    fn saturating_cast(self) -> u32 {
        self as u32
    }
}
