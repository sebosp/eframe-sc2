//! 1v1 Player Match Stats API
//!


use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct MatchStats {
    /// A set of filters for the players
    #[serde(skip)]
    pub request: MatchStatsReq,

    /// Contains the metadata related to the backend snapshot.
    #[serde(skip)]
    res: Option<poll_promise::Promise<MatchStatsRes>>,
}

/// Basic query request available for filtering replay players
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchStatsReq {
    /// The replay id
    #[serde(default)]
    pub replay_id: String,
    /// Minimum bound of the file date
    #[serde(default)]
    pub min_date: chrono::NaiveDate,
    /// Max bound of the file date
    #[serde(default)]
    pub max_date: chrono::NaiveDate,
    /// The map that the player played on
    #[serde(default)]
    pub map_title: String,
    /// A player that must have played a game
    #[serde(default)]
    pub player_1: String,
    /// A player that must have played a game
    #[serde(default)]
    pub player_2: String,
}

/// Basic query request available for filtering replay players
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchStatsRes {
    /// Metadata of the response
    pub meta: crate::meta::ResponseMeta,
    /// The data of the response
    pub data: Vec<PlayerMatchStats>,
}

/// A statistic that is min/max/average over a set of games
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GameStat {
    /// The minimum value of the statistic
    pub min: u32,
    /// The maximum value of the statistic
    pub max: u32,
    /// The average value of the statistic
    pub avg: f32,
}

/// A player match stats
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PlayerMatchStats {
    /// The name of the player
    pub player_name: String,
    /// The number of games played
    pub games_played: u32,
    /// The number of wins
    pub wins: u32,
    /// The number of losses
    /// The average game duration
    pub game_duration: u32,
}
