//! Common operations
//!

#[cfg(not(target_arch = "wasm32"))]
use polars::prelude::*;

use serde::{Deserialize, Serialize};

/// Contains the meta data of data frame results to be sent back to the clients.
#[derive(Debug, Serialize, Deserialize)]
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

/// Contains information related to the minimun, maximum date of the snapshot taken, the number of
/// files analyzed, the number of maps and the number of players.
#[derive(Debug, Serialize, Deserialize)]
pub struct BackendMeta {
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

impl Default for BackendMeta {
    fn default() -> Self {
        Self {
            min_date: chrono::NaiveDateTime::from_timestamp(0, 0),
            max_date: chrono::NaiveDateTime::from_timestamp(0, 0),
            num_files: 0,
            num_maps: 0,
            num_players: 0,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Converts a Dataframe into a String, this is expensive but useful for small results.
pub fn convert_df_to_json_data(df: &DataFrame) -> Result<String, crate::error::Error> {
    let mut buf = Vec::new();
    JsonWriter::new(&mut buf)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df.clone())?;
    Ok(String::from_utf8(buf)?)
}
