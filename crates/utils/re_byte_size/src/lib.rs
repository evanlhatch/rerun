//! Minimal stub for re_byte_size.

// Re-export the derive macro so #[derive(re_byte_size::SizeBytes)] works.
pub use re_byte_size_derive::*;

/// Size tracking trait. No blanket impl — only via `#[derive(SizeBytes)]`.
pub trait SizeBytes {
    /// Default returns 0 — override if tracking matters.
    fn heap_size_bytes(&self) -> u64 { 0 }
    const IS_POD: bool = false;
    fn total_size_bytes(&self) -> u64 { std::mem::size_of_val(self) as u64 }
}










/// A node in a memory usage tree. Minimal stub - used as trait in vendored code.
pub trait MemUsageNode {}

// ── Primitive type impls ──────────────────────────────────────────────────

macro_rules! impl_size_bytes_for_primitive {
    ($($t:ty),*) => {
        $(impl SizeBytes for $t {})*
    };
}

impl_size_bytes_for_primitive!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, bool, usize, isize);

// ── Standard type impls (needed by re_video and other vendored crates) ──────

impl SizeBytes for String {
    fn heap_size_bytes(&self) -> u64 {
        self.capacity() as u64
    }
}

impl<T: SizeBytes> SizeBytes for Vec<T> {
    fn heap_size_bytes(&self) -> u64 {
        // Capacity * element size for the backing buffer,
        // plus per-element heap for nested allocations.
        let capacity_bytes = self.capacity() as u64 * std::mem::size_of::<T>() as u64;
        let elements_bytes: u64 = self.iter().map(|e| e.heap_size_bytes()).sum();
        capacity_bytes + elements_bytes
    }
}

impl<T: SizeBytes, E: SizeBytes> SizeBytes for Result<T, E> {
    fn heap_size_bytes(&self) -> u64 {
        match self {
            Ok(t) => t.heap_size_bytes(),
            Err(e) => e.heap_size_bytes(),
        }
    }
}

/// A tree of memory usage nodes. Minimal stub.
pub struct MemUsageTree;

/// Captured memory usage tree. Minimal stub trait (used as trait bound in vendored code).
pub trait MemUsageTreeCapture {
    /// Capture a memory usage tree.
    fn capture_mem_usage_tree(&self) -> MemUsageTree;
}



impl<T: SizeBytes> SizeBytes for Option<T> {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        match self {
            Some(v) => v.heap_size_bytes(),
            None => 0,
        }
    }
}

/// ── parking_lot guard impls (needed for re_log_types) ──────────

impl<T> SizeBytes for parking_lot::RwLockReadGuard<'_, T>
where
    T: SizeBytes + ?Sized,
{
    fn heap_size_bytes(&self) -> u64 {
        (**self).heap_size_bytes()
    }
}

impl<T> SizeBytes for parking_lot::RwLockWriteGuard<'_, T>
where
    T: SizeBytes + ?Sized,
{
    fn heap_size_bytes(&self) -> u64 {
        (**self).heap_size_bytes()
    }
}

impl<T> SizeBytes for parking_lot::MutexGuard<'_, T>
where
    T: SizeBytes + ?Sized,
{
    fn heap_size_bytes(&self) -> u64 {
        (**self).heap_size_bytes()
    }
}

impl SizeBytes for arrow::array::FixedSizeBinaryArray {
    fn heap_size_bytes(&self) -> u64 {
        self.values().len() as u64
    }
}

// ── Arrow type impls (needed by re_chunk_store, re_entity_db, re_sorbet) ────

impl SizeBytes for arrow::array::RecordBatch {
    fn heap_size_bytes(&self) -> u64 {
        self.columns().iter().map(|c| c.get_array_memory_size() as u64).sum()
    }
}

impl SizeBytes for arrow::datatypes::Schema {
    fn heap_size_bytes(&self) -> u64 { 0 }
}

impl<T: arrow::buffer::ArrowNativeType> SizeBytes for arrow::buffer::ScalarBuffer<T> {
    fn heap_size_bytes(&self) -> u64 {
        std::mem::size_of_val(self.as_slice()) as u64
    }
}

impl SizeBytes for dyn arrow::array::Array {
    fn heap_size_bytes(&self) -> u64 {
        self.get_array_memory_size() as u64
    }
}

impl SizeBytes for arrow::array::GenericListArray<i32> {
    fn heap_size_bytes(&self) -> u64 {
        self.get_array_memory_size() as u64
    }
}

// ── std collection impls ──────────────────────────────────────────────────
impl<K: SizeBytes, V: SizeBytes, S> SizeBytes for std::collections::HashMap<K, V, S> {
    fn heap_size_bytes(&self) -> u64 {
        self.iter().map(|(k, v)| k.heap_size_bytes() + v.heap_size_bytes()).sum()
    }
}

impl<T: SizeBytes> SizeBytes for std::collections::BTreeSet<T> {
    fn heap_size_bytes(&self) -> u64 {
        self.iter().map(|e| e.heap_size_bytes()).sum()
    }
}

impl<T: SizeBytes, S> SizeBytes for std::collections::HashSet<T, S> {
    fn heap_size_bytes(&self) -> u64 {
        self.iter().map(|e| e.heap_size_bytes()).sum()
    }
}

impl<T: SizeBytes> SizeBytes for parking_lot::RwLock<T> {
    fn heap_size_bytes(&self) -> u64 {
        self.data_ptr().heap_size_bytes()
    }
}

impl<T: SizeBytes> SizeBytes for parking_lot::Mutex<T> {
    fn heap_size_bytes(&self) -> u64 {
        self.data_ptr().heap_size_bytes()
    }
}

