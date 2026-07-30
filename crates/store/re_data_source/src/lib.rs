//! Stub: re_data_source — flatland vendor does not load from external sources.
//! Provides LogDataSource with the correct types so re_viewer_context compiles.

use std::path::PathBuf;
use std::sync::Arc;

use re_uri::external::url::Url;
use re_uri::{DatasetSegmentUri, ProxyUri};

/// Stub — no external data sources are actually loaded in flatland vendor.
#[derive(Clone, Debug)]
pub enum LogDataSource {
    FilePath {
        path: PathBuf,
        file_source: re_log_types::FileSource,
        open_behavior: OpenBehavior,
    },
    FileContents {
        contents: Arc<[u8]>,
        file_source: re_log_types::FileSource,
        open_behavior: OpenBehavior,
    },
    FileHandle {
        path: PathBuf,
        file_source: re_log_types::FileSource,
        open_behavior: OpenBehavior,
    },
    HttpUrl {
        url: Url,
        open_behavior: OpenBehavior,
    },
    Stdin,
    RedapDatasetSegment {
        uri: DatasetSegmentUri,
        open_behavior: OpenBehavior,
    },
    RedapProxy(ProxyUri),
}

impl LogDataSource {
    pub fn from_uri(
        _source: re_log_types::FileSource,
        url: &str,
        _options: &FromUriOptions,
    ) -> Result<Self, anyhow::Error> {
        if url.starts_with("http") {
            Ok(Self::HttpUrl {
                url: url.parse()?,
                open_behavior: OpenBehavior::default(),
            })
        } else {
            Ok(Self::FilePath {
                path: url.into(),
                file_source: re_log_types::FileSource::Uri,
                open_behavior: OpenBehavior::default(),
            })
        }
    }
}

/// Stub — controls how a data source opens.
#[derive(Clone, Debug, Default)]
pub enum OpenBehavior {
    #[default]
    Default,
    ReplaceCurrent,
}

#[derive(Clone, Debug, Default)]
pub struct FromUriOptions;

impl FromUriOptions {
    pub fn accept_extensionless_http(self, _accept: bool) -> Self { self }
}
