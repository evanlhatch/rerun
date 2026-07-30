//! Stub: re_redap_client — flatland vendor does not use Redap gRPC.
//!
//! Only provides ConnectionRegistry + ConnectionRegistryHandle as empty
//! types so AppContext compiles. All gRPC/tonic/opentelemetry deps removed.

use std::sync::Arc;

/// Stub — no connections are tracked.
#[derive(Default)]
pub struct ConnectionRegistry;

/// Handle to a [`ConnectionRegistry`] — Arc for shared ownership.
pub type ConnectionRegistryHandle = Arc<ConnectionRegistry>;

impl ConnectionRegistry {
    pub fn new_without_stored_credentials() -> ConnectionRegistryHandle {
        Arc::new(Self::default())
    }
}
