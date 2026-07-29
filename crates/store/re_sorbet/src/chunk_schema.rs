use re_log_types::EntityPath;

/// Describes the schema of a chunk.
#[derive(Debug, Clone)]
pub struct ChunkSchema {
    pub entity_path: EntityPath,
}

impl ChunkSchema {
    pub fn new(entity_path: EntityPath) -> Self {
        Self { entity_path }
    }

    pub fn entity_path(&self) -> &EntityPath {
        &self.entity_path
    }
}
