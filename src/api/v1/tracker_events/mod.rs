//! The Tracker Events interaction

#[cfg(not(target_arch = "wasm32"))]
pub mod dataframe;

//#[cfg(not(target_arch = "wasm32"))]
//pub mod server;

pub mod ui;

use serde::{Deserialize, Serialize};

/// A query for the Position of the Unit Born TrackerEvents
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UnitBornPosReq {
    /// The name of the player
    pub player: Option<String>,
    /// The name of the unit
    pub unit_type_name: String,
    /// An optional game loop of the event
    pub game_loop: Option<i64>,
    /// The file sha256 hash of the replay
    pub file_hash: String,
}

/// Basic query response available for filtering replay maps
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UnitBornPosRes {
    /// Metadata of the response
    pub meta: crate::meta::ResponseMeta,
    /// The data of the response
    pub data: Vec<UnitBornPosEvent>,
}

/// A positioned unit born event
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UnitBornPosEvent {
    /// The event
    pub unit_type_name: String,
    /// The X position of the event
    pub x: f32,
    /// The Y position of the event
    pub y: f32,
    /// The game loop of the event
    pub game_loop: i64,
}
