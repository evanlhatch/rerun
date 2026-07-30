//! Stub: re_protos — flatland vendor does not use protobuf transport.
//!
//! All gRPC/protobuf types are stubbed. This removes prost/tonic/opentelemetry
//! from the dependency tree. The only real usage (re_log_channel's
//! GetViewerStateResponse/SetTimeCursorResponse) is gated behind the
//! "transport" feature on re_log_channel.

pub mod external {
    pub use prost;
}

pub mod common {
    pub mod v1alpha1 {
        pub mod ext {
            #[derive(Debug, Clone)]
            pub struct StoreIdMissingApplicationIdError;
            #[derive(Debug, Clone)]
            pub struct StoreIdFromProtoError;
        }

        #[derive(Debug, Clone, Default)]
        pub struct Schema;

        #[derive(Debug, Clone, Copy, Default)]
        pub enum Compression {
            #[default]
            None,
        }
    }
}

pub mod log_msg {
    pub mod v1alpha1 {
        #[derive(Debug, Clone, Default)]
        pub struct ArrowMsg;
        #[derive(Debug, Clone, Default)]
        pub struct RrdFooter {
            pub manifests: Vec<()>,
        }
        #[derive(Debug, Clone, Default)]
        pub struct RrdManifest;
        #[derive(Debug, Clone, Default)]
        pub struct SetStoreInfo;
        #[derive(Debug, Clone, Default)]
        pub struct BlueprintActivationCommand;

        pub mod log_msg {
            #[derive(Debug, Clone)]
            pub enum Msg {
                ArrowMsg(ArrowMsg),
                BlueprintActivationCommand(BlueprintActivationCommand),
                SetStoreInfo(SetStoreInfo),
            }
        }

        impl RrdFooter {
            pub fn from_rrd_bytes(_bytes: &[u8]) -> Result<Self, crate::StubError> {
                Err(crate::StubError)
            }
        }

        impl ArrowMsg {
            pub fn from_rrd_bytes(_bytes: &[u8]) -> Result<Self, crate::StubError> {
                Err(crate::StubError)
            }
        }

        impl log_msg::Msg {
            pub fn decode(_bytes: &[u8]) -> Result<Option<Self>, crate::StubError> {
                Err(crate::StubError)
            }
        }

        impl SetStoreInfo {
            pub fn from_rrd_bytes(_bytes: &[u8]) -> Result<Self, crate::StubError> {
                Err(crate::StubError)
            }
        }
    }
}

pub mod sdk_comms {
    pub mod v1alpha1 {
        #[derive(Debug, Clone, Default)]
        pub struct GetViewerStateResponse;
        #[derive(Debug, Clone, Default)]
        pub struct SetTimeCursorResponse;
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("re_protos stub: protobuf transport not available in flatland vendor")]
pub struct StubError;

#[macro_export]
macro_rules! missing_field {
    ($type_:ty, $field:expr) => {
        anyhow::anyhow!("missing field {} in {}", $field, stringify!($type_))
    };
}

#[macro_export]
macro_rules! invalid_field {
    ($type_:ty, $field:expr) => {
        anyhow::anyhow!("invalid field {} in {}", $field, stringify!($type_))
    };
}
