use arrow::array::{ArrayRef, RecordBatch as ArrowRecordBatch};
use re_chunk::Chunk;
use re_log_types::EntityPath;

use crate::ChunkSchema;

/// A serialized chunk — schema + arrow record batch.
#[derive(Debug, Clone)]
pub struct ChunkBatch {
    pub schema: ChunkSchema,
    pub columns: Vec<ArrayRef>,
    pub num_rows: usize,
}

impl ChunkBatch {
    pub fn try_new(schema: ChunkSchema, columns: Vec<ArrayRef>, num_rows: usize) -> Result<Self, crate::SorbetError> {
        Ok(Self {
            schema,
            columns,
            num_rows,
        })
    }

    pub fn schema(&self) -> &ChunkSchema {
        &self.schema
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn columns(&self) -> &[ArrayRef] {
        &self.columns
    }

    pub fn into_columns(self) -> Vec<ArrayRef> {
        self.columns
    }

    pub fn arrow_batch(&self) -> ArrowRecordBatch {
        let schema = arrow::datatypes::Schema::new(
            self.columns
                .iter()
                .map(|arr| arrow::datatypes::Field::new("col", arr.data_type().clone(), true))
                .collect::<Vec<_>>(),
        );
        ArrowRecordBatch::try_new(Arc::new(schema), self.columns.clone(), self.num_rows)
            .expect("valid record batch")
    }
}

impl TryFrom<&Chunk> for ChunkBatch {
    type Error = crate::SorbetError;

    fn try_from(_chunk: &Chunk) -> Result<Self, Self::Error> {
        Err(crate::SorbetError::UnsupportedSource("ChunkBatch::try_from(&Chunk) not implemented".into()))
    }
}

impl std::fmt::Display for ChunkBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChunkBatch({} rows, {} cols)", self.num_rows, self.columns.len())
    }
}
