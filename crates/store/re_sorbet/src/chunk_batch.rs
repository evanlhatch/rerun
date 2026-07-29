use re_log_types::EntityPath;

use crate::ChunkSchema;

/// A serialized chunk — schema + data.
#[derive(Debug, Clone)]
pub struct ChunkBatch {
    pub schema: ChunkSchema,
    pub num_rows: usize,
}

impl ChunkBatch {
    pub fn try_new(schema: ChunkSchema) -> Result<Self, String> {
        Ok(Self {
            schema,
            num_rows: 0,
        })
    }

    pub fn schema(&self) -> &ChunkSchema {
        &self.schema
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }
}

impl std::fmt::Display for ChunkBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChunkBatch({} rows)", self.num_rows)
    }
}
