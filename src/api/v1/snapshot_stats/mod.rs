//! Provides information about the analyzed game collection.

pub mod dataframe;
pub mod server;

use serde::{Deserialize, Serialize};

/// Contains metadata information related to the minimun, maximum date of the snapshot taken, the number of
/// files analyzed, the number of maps and the number of players in the analyzed collection
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SnapshotStats {
    /// Metadata of the response
    pub meta: crate::meta::ResponseMeta,
    /// The size of the IPC files
    pub directory_size: u64,
    /// The time of modification of the details IPC file.
    pub date_modified: std::time::SystemTime,
}
