//! Common operations
//!

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

pub fn convert_df_to_json_data(df: &DataFrame) -> Result<String, crate::error::Error> {
    let mut buf = Vec::new();
    JsonWriter::new(&mut buf)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df.clone())?;
    Ok(String::from_utf8(buf)?)
}
