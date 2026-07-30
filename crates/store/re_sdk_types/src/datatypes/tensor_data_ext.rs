use crate::datatypes::TensorBuffer::{self, *};
use crate::tensor_data::TensorImageLoadError;

use super::TensorData;

impl Default for TensorData {
    fn default() -> Self {
        use arrow::buffer::ScalarBuffer;
        Self {
            shape: ScalarBuffer::from(vec![]),
            names: None,
            buffer: TensorBuffer::default(),
        }
    }
}

impl TensorData {
    /// Update the names of the dimensions.
    pub fn with_dim_names(
        mut self,
        names: impl IntoIterator<Item = impl Into<re_types_core::ArrowString>>,
    ) -> Self {
        self.names = Some(names.into_iter().map(Into::into).collect());
        self
    }

    /// Create a TensorData from an image (stub: creates empty tensor).
    #[cfg(feature = "image")]
    pub fn from_image(
        _image: impl Into<image::DynamicImage>,
    ) -> Result<Self, TensorImageLoadError> {
        use arrow::buffer::ScalarBuffer;
        Ok(Self {
            shape: ScalarBuffer::from(vec![0u64, 0u64, 0u64]),
            names: None,
            buffer: U8(ScalarBuffer::from(vec![])),
        })
    }

    /// Create a TensorData from a DynamicImage (stub: creates empty tensor).
    #[cfg(feature = "image")]
    pub fn from_dynamic_image(
        _image: image::DynamicImage,
    ) -> Result<Self, TensorImageLoadError> {
        use arrow::buffer::ScalarBuffer;
        Ok(Self {
            shape: ScalarBuffer::from(vec![0u64, 0u64, 0u64]),
            names: None,
            buffer: U8(ScalarBuffer::from(vec![])),
        })
    }
}
