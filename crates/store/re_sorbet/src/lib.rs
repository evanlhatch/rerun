//! Shim: re_sorbet — stub error types for vendored re_chunk.

use thiserror::Error;

#[derive(Clone, Debug, Error)]
#[error("Index column error")]
pub struct IndexColumnError;

#[derive(Clone, Debug, Error)]
#[error("Mismatched chunk schema: {0}")]
pub struct MismatchedChunkSchemaError(pub String);

/// Minimal ChunkSchema stub — wraps a SorbetSchema + EntityPath.
#[derive(Debug, Clone)]
pub struct ChunkSchema {
    _private: (),
}

impl ChunkSchema {
    pub fn new(sorbet_schema: crate::SorbetSchema, entity_path: &re_log_types::EntityPath) -> Result<Self, crate::SorbetError> {
        let _ = (sorbet_schema, entity_path);
        Ok(Self { _private: () })
    }

    pub fn sorbet_schema(&self) -> &crate::SorbetSchema {
        unimplemented!("ChunkSchema stub")
    }

    pub fn entity_path(&self) -> &re_log_types::EntityPath {
        unimplemented!("ChunkSchema stub")
    }
}

/// Minimal ChunkBatch stub — wraps a SorbetBatch + EntityPath.
#[derive(Debug, Clone)]
pub struct ChunkBatch {
    _private: (),
}

impl ChunkBatch {
    pub fn try_new(batch: crate::SorbetBatch, entity_path: &re_log_types::EntityPath) -> Result<Self, crate::SorbetError> {
        let _ = (batch, entity_path);
        Ok(Self { _private: () })
    }
}

impl TryFrom<&re_chunk::Chunk> for ChunkBatch {
    type Error = crate::SorbetError;
    fn try_from(_chunk: &re_chunk::Chunk) -> Result<Self, Self::Error> {
        Ok(Self { _private: () })
    }
}


#[derive(Clone, Debug, Error)]
#[error("Sorbet schema error: {0}")]
pub struct SorbetError(pub String);

/// Placeholder — not used at the stub level.
#[derive(Clone, Debug, Error)]
#[error("Dataframe to chunks error")]
pub struct DataframeToChunksError;

/// Describes a component column selection.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentColumnSelector {
    /// The path of the entity.
    pub entity_path: re_log_types::EntityPath,
    /// The string representation of the component identifier.
    pub component: String,
}

impl ComponentColumnSelector {
    pub fn column_name(&self) -> String {
        format!("{}:{}", self.entity_path, self.component)
    }
}

impl std::fmt::Display for ComponentColumnSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.entity_path, self.component)
    }
}

/// Descriptors for chunk columns.
#[derive(Clone, Debug)]
pub struct ChunkColumnDescriptors;

/// Descriptor for a component column.
#[derive(Clone, Debug)]
pub struct ComponentColumnDescriptor {
    /// The entity path this column belongs to.
    pub entity_path: re_log_types::EntityPath,
    /// The component identifier (string form).
    pub component: String,
    /// Archetype name.
    pub archetype: Option<String>,
    /// Component type (numeric id).
    pub component_type: u32,
    /// Whether the column is static data.
    pub is_static: bool,
    /// Whether the column is a tombstone (Clear component).
    pub is_tombstone: bool,
    /// Whether the column is semantically empty.
    pub is_semantically_empty: bool,
    /// The Arrow datatype of the inner storage.
    pub store_datatype: arrow::datatypes::DataType,
}

/// Descriptor for an index column.
#[derive(Clone, Debug)]
pub struct IndexColumnDescriptor;

/// Descriptor for a row-id column.
#[derive(Clone, Debug)]
pub struct RowIdColumnDescriptor;


/// The kind of a column.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColumnKind {
    RowId,
    Time,
    Component,
}

/// Describes a column selection.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ColumnSelector {
    RowId,
    Time(TimeColumnSelector),
    Component(ComponentColumnSelector),
}

/// Select a time column.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TimeColumnSelector {
    /// The timeline name.
    pub timeline: re_log_types::TimelineName,
}


/// Batch type for columnar data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BatchType {
    /// A component batch.
    Component,
    /// A time batch.
    Time,
    /// A row-id batch.
    RowId,
}


// ── Flatland-vendor additions ──

/// Create a Schema from IPC bytes (stub — returns empty schema).
pub fn migrated_schema_from_ipc(_data: &[u8]) -> Result<std::sync::Arc<arrow::datatypes::Schema>, arrow::error::ArrowError> {
    Ok(std::sync::Arc::new(arrow::datatypes::Schema::empty()))
}

/// Serialize a Schema to IPC bytes (stub — returns empty vec).
pub fn ipc_from_schema(_schema: &arrow::datatypes::Schema) -> Result<Vec<u8>, arrow::error::ArrowError> {
    Ok(Vec::new())
}
