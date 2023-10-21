//! Map frequency related queries

pub mod ui;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
pub mod dataframe;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

/// Basic query request available for filtering replay maps
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListDetailsMapFreqReq {
    /// The title of the map
    pub title: Option<String>,
    /// A player that must have played in the game
    pub player: Option<String>,
}

/// Basic query response available for filtering replay maps
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListDetailsMapFreqRes {
    /// Metadata of the response
    pub meta: crate::meta::ResponseMeta,
    /// The data of the response
    pub data: Vec<MapFrequency>,
}

/// Basic response for map frequency
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MapFrequency {
    /// Teh name of the map
    pub title: String,
    /// The amount of replays on this map
    pub count: u32,
}
