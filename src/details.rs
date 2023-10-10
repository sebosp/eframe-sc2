//! Contains the details module
//!

use crate::common::*;
use serde::{Deserialize, Serialize};

pub const DETAILS_IPC: &str = "details.ipc";

/// Basic query request available for filtering replay maps
#[derive(Debug, Serialize, Deserialize)]
pub struct ListReplayReq {
    /// The title of the map
    title: Option<String>,
    /// A player that must have played in the game
    player: Option<String>,
}

/// Basic query response available for filtering  replay maps
#[derive(Debug, Serialize, Deserialize)]
pub struct ListReplayRes {
    /// Metadata of the response
    meta: ResponseMeta,
    /// The data of the response
    data: Vec<MapFrequency>,
}

/// Basic response for map frequency
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MapFrequency {
    /// Teh name of the map
    title: String,
    /// The amount of replays on this map
    count: u32,
}
