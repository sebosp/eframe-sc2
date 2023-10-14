//! Contains the details module
//!

use crate::common::*;
use serde::{Deserialize, Serialize};

pub const DETAILS_IPC: &str = "details.ipc";

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
    pub meta: ResponseMeta,
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
