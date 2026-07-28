//! Shim: minimal stub for re_log_encoding — Arrow message encoding/decoding.

use arrow::array::RecordBatch as ArrowRecordBatch;
use re_log_types::LogMsg;

/// Decode a LogMsg from a stream of bytes (stub — always returns None).
pub fn decode(_bytes: &[u8]) -> Option<LogMsg> {
    None
}

/// Encode a LogMsg to bytes (stub — returns empty vec).
pub fn encode(_msg: &LogMsg) -> Vec<u8> {
    Vec::new()
}

/// Decode a LogMsg from a RecordBatch (stub).
pub fn decode_from_record_batch(_batch: &ArrowRecordBatch) -> Option<LogMsg> {
    None
}


use std::sync::Arc;
use re_log_types::StoreId;

/// Manifest of an .rrd file. Minimal stub.
#[derive(Clone, Debug)]
pub struct RrdManifest {
    pub store_id: StoreId,
}

/// Raw manifest data. Minimal stub.
#[derive(Clone, Debug)]
pub struct RawRrdManifest;

/// Provider of chunks. Minimal stub trait.
pub trait ChunkProvider: Send + Sync + std::fmt::Debug {}

/// Temporal map entry for an RrdManifest. Minimal stub.
#[derive(Clone, Debug)]
pub struct RrdManifestTemporalMapEntry;


/// Decoder for Rerun log streams. Minimal stub.
#[derive(Clone, Debug)]
pub struct Decoder;

impl Decoder {
    /// Decode eagerly from a reader (synchronous).
    pub fn decode_eager<T: std::io::Read>(_reader: T) -> Result<Vec<re_log_types::LogMsg>, String> {
        Ok(Vec::new())
    }
    
    /// Decode eagerly from an async reader (stub).
    pub async fn decode_eager_async<T: futures::io::AsyncRead + Unpin>(_reader: T) -> Result<Vec<re_log_types::LogMsg>, String> {
        Ok(Vec::new())
    }
}


/// A default empty ChunkProvider implementation.
#[derive(Clone, Debug)]
pub struct EmptyChunkProvider;
impl ChunkProvider for EmptyChunkProvider {}
