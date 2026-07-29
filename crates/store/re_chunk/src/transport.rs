use arrow::array::RecordBatch as ArrowRecordBatch;

use crate::{Chunk, ChunkError, ChunkResult};

// ---

impl Chunk {
    /// Stub: flatland vendor does not support sorbet serialization.
    pub fn to_record_batch(&self) -> ChunkResult<ArrowRecordBatch> {
        Err(ChunkError::Malformed {
            reason: "flatland vendor: to_record_batch not implemented".into(),
        })
    }

    /// Stub: flatland vendor does not support sorbet serialization.
    pub fn to_chunk_batch(&self) -> ChunkResult<re_sorbet::ChunkBatch> {
        Err(ChunkError::Malformed {
            reason: "flatland vendor: to_chunk_batch not implemented".into(),
        })
    }

    /// Stub: flatland vendor does not support sorbet deserialization.
    pub fn from_chunk_record_batch(_batch: &ArrowRecordBatch) -> ChunkResult<Self> {
        Err(ChunkError::Malformed {
            reason: "flatland vendor: from_chunk_record_batch not implemented".into(),
        })
    }

    /// Stub: flatland vendor does not support sorbet deserialization.
    pub fn from_record_batch(
        _batch: &ArrowRecordBatch,
        _index: &re_sorbet::DataframeIndex,
        _entity_path: Option<&re_log_types::EntityPath>,
    ) -> ChunkResult<Vec<Self>> {
        Err(ChunkError::Malformed {
            reason: "flatland vendor: from_record_batch not implemented".into(),
        })
    }

    /// Stub: flatland vendor does not support sorbet deserialization.
    pub fn from_chunk_batch(_batch: &re_sorbet::ChunkBatch) -> ChunkResult<Self> {
        Err(ChunkError::Malformed {
            reason: "flatland vendor: from_chunk_batch not implemented".into(),
        })
    }
}

impl Chunk {
    /// Stub: flatland vendor does not support arrow message serialization.
    pub fn from_arrow_msg(_msg: &re_log_types::ArrowMsg) -> ChunkResult<Self> {
        Err(ChunkError::Malformed {
            reason: "flatland vendor: from_arrow_msg not implemented".into(),
        })
    }

    /// Stub: flatland vendor does not support arrow message serialization.
    pub fn to_arrow_msg(&self) -> ChunkResult<re_log_types::ArrowMsg> {
        Err(ChunkError::Malformed {
            reason: "flatland vendor: to_arrow_msg not implemented".into(),
        })
    }
}
