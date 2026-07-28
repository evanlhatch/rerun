//! Minimal stub for re_byte_size.

// Re-export the derive macro so #[derive(re_byte_size::SizeBytes)] works.
pub use re_byte_size_derive::*;

/// Size tracking trait. No blanket impl — only via `#[derive(SizeBytes)]`.
pub trait SizeBytes {
    const IS_POD: bool = false;
    fn heap_size_bytes(&self) -> u64 { 0 }
    fn total_size_bytes(&self) -> u64 { std::mem::size_of_val(self) as u64 }
}










/// A node in a memory usage tree. Minimal stub - used as trait in vendored code.
pub trait MemUsageNode {}

/// A tree of memory usage nodes. Minimal stub.
pub struct MemUsageTree;

/// Captured memory usage tree. Minimal stub trait (used as trait bound in vendored code).
pub trait MemUsageTreeCapture {
    /// Capture a memory usage tree.
    fn capture_mem_usage_tree(&self) -> MemUsageTree;
}

