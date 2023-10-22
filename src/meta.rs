//! Meta data response for http responses

use serde::{Deserialize, Serialize};

/// Contains the meta data of data frame results to be sent back to the clients.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ResponseMeta {
    /// Wether there was an error or not
    pub status: String,
    /// The total number of rows in the data frame
    pub total: usize,
    /// The time at which the snapshot was taken
    pub snapshot_epoch: u64,
    /// A potential message in case of error. TODO: Refactor to Error specific type
    pub message: String,
}

/// Contains metadata information related to the minimun, maximum date of the snapshot taken, the number of
/// files analyzed, the number of maps and the number of players in the analyzed collection
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnalyzedSnapshotMeta {
    /// The minimum date of the snapshot taken
    pub min_date: chrono::NaiveDateTime,
    /// The maximum date of the snapshot taken
    pub max_date: chrono::NaiveDateTime,
    /// The number of files analyzed
    pub num_files: usize,
    /// The number of maps
    pub num_maps: usize,
    /// The number of players
    pub num_players: usize,
}
