use arrow::datatypes::DataType;
use re_chunk::ChunkId;
use re_log_types::EntityPath;

use crate::SorbetSchema;

/// Describes the schema of a chunk: its entity path and column layout.
#[derive(Debug, Clone)]
pub struct ChunkSchema {
    pub chunk_id: ChunkId,
    pub entity_path: EntityPath,
    pub sorbet_schema: SorbetSchema,
}

impl ChunkSchema {
    pub fn new(chunk_id: ChunkId, entity_path: EntityPath, sorbet_schema: SorbetSchema) -> Self {
        Self {
            chunk_id,
            entity_path,
            sorbet_schema,
        }
    }

    pub fn sorbet_schema(&self) -> &SorbetSchema {
        &self.sorbet_schema
    }

    pub fn entity_path(&self) -> &EntityPath {
        &self.entity_path
    }

    pub fn chunk_id(&self) -> &ChunkId {
        &self.chunk_id
    }

    /// Number of columns in this chunk schema.
    pub fn num_columns(&self) -> usize {
        self.sorbet_schema.num_columns()
    }

    /// Get the data type of a column by index.
    pub fn column_data_type(&self, _col: usize) -> Option<&DataType> {
        None
    }
}
