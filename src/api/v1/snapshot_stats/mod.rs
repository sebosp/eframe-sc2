//! Provides information about the analyzed game collection.

#[cfg(not(target_arch = "wasm32"))]
pub mod dataframe;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

use serde::{Deserialize, Serialize};

/// Contains metadata information related to the minimun, maximum date of the snapshot taken, the number of
/// files analyzed, the number of maps and the number of players in the analyzed collection
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotStats {
    /// Metadata of the response
    pub meta: crate::meta::ResponseMeta,
    /// The size of the IPC files
    pub directory_size: u64,
    /// The time of modification of the details IPC file.
    pub date_modified: std::time::SystemTime,
}

impl Default for SnapshotStats {
    fn default() -> Self {
        SnapshotStats {
            meta: crate::meta::ResponseMeta::default(),
            directory_size: 0,
            date_modified: std::time::SystemTime::UNIX_EPOCH,
        }
    }
}
