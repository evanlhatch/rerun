//! Stub when dataframe feature is disabled — no datafusion dependency.

use re_log_types::TableId;

#[derive(Default)]
pub struct TableStore;

impl TableStore {
    pub const TABLE_NAME: &'static str = "__table__";

    pub fn total_size_bytes(&self) -> u64 { 0 }
}

pub type TableStores = ahash::HashMap<TableId, TableStore>;
