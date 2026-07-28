//! Minimal re_arrow_util shim — stubs for vendored store crates.
//! Matches original Rerun's ArrowArrayDowncastRef trait.

use arrow::array::{Array, ArrayRef, BooleanArray, UInt64Array, ArrayData};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

// ── ArrowArrayDowncastRef trait (matches original Rerun API) ─────
//
// This trait enables `array.downcast_array_ref::<T>()` on any `&dyn Array`.

pub trait ArrowArrayDowncastRef<'a>: 'a {
    fn downcast_array_ref<T: Array + 'static>(self) -> Option<&'a T>;
    fn try_downcast_array_ref<T: Array + 'static>(self) -> Result<&'a T, ArrowError>;
    fn try_downcast_array<T: Array + Clone + 'static>(self) -> Result<T, ArrowError>;
}

impl<'a> ArrowArrayDowncastRef<'a> for &'a dyn Array {
    fn downcast_array_ref<T: Array + 'static>(self) -> Option<&'a T> {
        self.as_any().downcast_ref()
    }

    fn try_downcast_array_ref<T: Array + 'static>(self) -> Result<&'a T, ArrowError> {
        self.downcast_array_ref::<T>().ok_or_else(|| {
            ArrowError::CastError(format!(
                "Failed to downcast array of type {} to {}",
                self.data_type(),
                std::any::type_name::<T>(),
            ))
        })
    }

    fn try_downcast_array<T: Array + Clone + 'static>(self) -> Result<T, ArrowError> {
        self.downcast_array_ref::<T>()
            .cloned()
            .ok_or_else(|| {
                ArrowError::CastError(format!(
                    "Failed to downcast array of type {} to {}",
                    self.data_type(),
                    std::any::type_name::<T>(),
                ))
            })
    }
}

/// Free-function helpers. Use via `re_arrow_util::downcast_array_ref::<T>(&array)`.

pub fn downcast_array_ref<'a, T: Array + 'static>(array: &'a dyn Array) -> Option<&'a T> {
    array.as_any().downcast_ref()
}

// ── Array comparison & utilities ─────────────────────────────────

pub fn ensure_similar(a: &dyn Array, b: &dyn Array) -> anyhow::Result<()> {
    if a.data_type() != b.data_type() || a.len() != b.len() {
        Err(anyhow::anyhow!("Array mismatch: {:?} len={} vs {:?} len={}", a.data_type(), a.len(), b.data_type(), b.len()))
    } else {
        Ok(())
    }
}

pub fn format_record_batch_with_width(
    batch: &RecordBatch,
    _width: usize,
) -> Vec<String> {
    vec![format!(
        "RecordBatch({} rows, {} cols)",
        batch.num_rows(),
        batch.num_columns()
    )]
}

// ── Array slicing / filtering (stubs) ────────────────────────────

pub fn deep_slice_array(arr: &dyn Array, start: usize, len: usize) -> ArrayRef {
    arr.slice(start, len)
}

pub fn deep_slice_array_start_len(arr: &dyn Array, start: usize, len: usize) -> ArrayRef {
    arr.slice(start, len)
}

pub fn filter_array(values: &dyn Array, _filter: &BooleanArray) -> ArrayRef {
    arrow::array::make_array(values.to_data())
}

pub fn take_array(values: &dyn Array, _indices: &dyn Array) -> ArrayRef {
    arrow::array::make_array(values.to_data())
}

pub fn widen_binary_arrays(arr: &dyn Array) -> ArrayRef {
    arrow::array::make_array(arr.to_data())
}

// ── Error types ─────────────────────────────────────────────────

#[derive(Debug)]
pub struct WrongDatatypeError {
    pub column_name: Option<String>,
    pub expected: String,
    pub actual: String,
}

impl std::fmt::Display for WrongDatatypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wrong datatype: expected {}, got {}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for WrongDatatypeError {}


/// Concatenate multiple arrow arrays.
pub fn concat_arrays(arrays: &[&dyn arrow::array::Array]) -> arrow::error::Result<arrow::array::ArrayRef> {
    arrow::compute::kernels::concat::concat(arrays)
}

/// Returns true if a ListArray is semantically empty (all nulls or zero-length lists).
pub fn is_list_array_semantically_empty(list_array: &arrow::array::ListArray) -> bool {
    if list_array.is_empty() {
        return true;
    }
    if list_array.null_count() == list_array.len() {
        return true;
    }
    // Check if all offsets are equal (meaning all sub-arrays are zero-length)
    let offsets = list_array.offsets();
    if offsets.len() >= 2 {
        let first = offsets[0];
        let all_zero = offsets.iter().all(|o| *o == first);
        if all_zero {
            return true;
        }
    }
    false
}


/// Convert an Arrow array into a reference-counted array ref.
pub fn into_arrow_ref(arr: impl Into<arrow::array::ArrayRef>) -> arrow::array::ArrayRef {
    arr.into()
}
