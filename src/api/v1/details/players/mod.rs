//! Player count related queries
//!
use s2protocol::details::ToonNameDetails;
use urlencoding::encode;

pub mod ui;

#[cfg(not(target_arch = "wasm32"))]
pub mod dataframe;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

use serde::{Deserialize, Serialize};

/// Basic query request available for filtering replay players
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListDetailsPlayerReq {
    /// A player that must have played in the game
    #[serde(default)]
    pub name: String,
    /// Part of the file name
    #[serde(default)]
    pub file_name: String,
    /// Part of the SHA256 hash
    #[serde(default)]
    pub file_hash: String,
    /// Minimum bound of the file date
    #[serde(default)]
    pub file_min_date: chrono::NaiveDate,
    /// Max bound of the file date
    #[serde(default)]
    pub file_max_date: chrono::NaiveDate,
}

impl Default for ListDetailsPlayerReq {
    fn default() -> Self {
        Self {
            name: Default::default(),
            file_name: Default::default(),
            file_hash: Default::default(),
            file_min_date: Self::default_min_date(),
            file_max_date: Self::default_max_date(),
        }
    }
}

impl ListDetailsPlayerReq {
    /// Returns a new instance of the request with the unescaped values
    pub fn from_escaped(self) -> Self {
        Self {
            name: urlencoding::decode(&self.name)
                .unwrap_or_default()
                .to_string(),
            file_name: urlencoding::decode(&self.file_name)
                .unwrap_or_default()
                .to_string(),
            file_hash: urlencoding::decode(&self.file_hash)
                .unwrap_or_default()
                .to_string(),
            file_min_date: self.file_min_date,
            file_max_date: self.file_max_date,
        }
    }

    /// Creates a default min date for dropdowns on date filters.
    pub fn default_min_date() -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()
    }

    /// Creates a default max date for dropdowns on date filters.
    pub fn default_max_date() -> chrono::NaiveDate {
        chrono::Local::now().naive_local().date()
    }
}

/// Basic query response available for filtering replay players
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ListDetailsPlayerRes {
    /// Metadata of the response
    pub meta: crate::meta::ResponseMeta,
    /// The data of the response
    pub data: Vec<PlayerStats>,
}

/// Basic response for playper frequency
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    /// The clan of the player
    pub clan: Option<String>,
    /// The name of the player
    #[serde(rename = "player_name")]
    pub name: String,
    /// The amount of replays for this player
    pub count: u32,
    /// The minimum date of the snapshot taken
    pub min_date: chrono::NaiveDateTime,
    /// The maximum date of the snapshot taken
    pub max_date: chrono::NaiveDateTime,
    /// The latest sha256 hash of the player
    pub latest_replay_sha: String,
    /// The top frequency maps for this player
    pub top_maps: Vec<String>,
    /// The race stats
    #[serde(skip)]
    pub race_stats: Vec<PlayerRaceStats>,
    /// Toon related information
    pub toon: ToonNameDetails,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayerRaceStats {
    /// The race played, zerg, protoss or terran
    pub race: u8,
    /// The amount of replays on the local snapshot
    pub count: u32,
    /// The number of wins on this race for this player
    pub wins: u32,
    /// The number of defeats for this player
    pub defeats: u32,
    /// The number of undecided on this race for this player
    pub undecided: u32,
    /// The number of ties on this race for this player
    pub ties: u32,
}

impl PlayerStats {
    /// A visible label for the player blizzard link
    pub fn blizzard_profile_link_title(&self) -> String {
        format!("{}/{}/{}", self.toon.region, self.toon.realm, self.toon.id)
    }
    /// Creates a link to access the player info on the battle.net website
    pub fn blizzard_profile_link_href(&self) -> String {
        format!(
            "https://starcraft2.blizzard.com/en-us/profile/{}/{}/{}",
            self.toon.region, self.toon.realm, self.toon.id
        )
    }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct SC2PlayerPicker {
    /// A set of filters for the players
    #[serde(skip)]
    request: ListDetailsPlayerReq,

    /// Contains the metadata related to the backend snapshot.
    #[serde(skip)]
    player_list: Option<poll_promise::Promise<ListDetailsPlayerRes>>,

    /// The selected player
    #[serde(skip)]
    pub selected_player: Option<PlayerStats>,
}

impl SC2PlayerPicker {
    async fn get_details_players(filters: ListDetailsPlayerReq) -> ListDetailsPlayerRes {
        let mut query_params: Vec<String> = vec![];
        query_params.push(format!("name={}", encode(&filters.name)));
        query_params.push(format!("file_name={}", encode(&filters.file_name)));
        query_params.push(format!("file_hash={}", encode(&filters.file_hash)));
        query_params.push(format!(
            "file_min_date={}",
            encode(&filters.file_min_date.to_string())
        ));
        query_params.push(format!(
            "file_max_date={}",
            encode(&filters.file_max_date.to_string())
        ));
        let query_url = format!("/api/v1/details/players?{}", query_params.join("&"));
        ehttp::fetch_async(ehttp::Request::get(query_url))
            .await
            .map(|response| serde_json::from_slice(&response.bytes).unwrap_or_default())
            .unwrap_or_default()
    }

    /// Requests the async operation to get the details of the players to the HTTP server.
    pub fn req_details_players(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            log::info!("Requesting details players");
            self.player_list = Some(poll_promise::Promise::spawn_local(
                Self::get_details_players(self.request.clone()),
            ));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            tracing::info!("Requesting details players");
            self.player_list = Some(poll_promise::Promise::spawn_async(
                Self::get_details_players(self.request.clone()),
            ));
        }
    }
}

// test module
#[cfg(test)]
mod tests {
    use super::*;
    use s2protocol::details::ToonNameDetails;

    #[test]
    fn test_serde_player_stats() {
        let example_str = r#"{"name":"Sazed","count":1852,"min_date":"2021-04-12T13:55:57.058","max_date":"2023-09-01T15:01:38.400", "latest_replay_sha": "whatevs",
    top_maps: ["Emerald City LE"]}"#;
        let example: PlayerStats = serde_json::from_str(example_str).unwrap();
        let example_toon: ToonNameDetails = ToonNameDetails {
            region: 1,
            realm: 1,
            id: 1,
        };
        assert_eq!(
            example,
            PlayerStats {
                name: "Sazed".to_string(),
                toon: example_toon,
                clan: None,
                race_stats: vec![],
                count: 1852,
                min_date: chrono::NaiveDate::from_ymd_opt(2021, 4, 12)
                    .unwrap()
                    .and_hms_milli_opt(13, 55, 57, 58)
                    .unwrap(),
                max_date: chrono::NaiveDate::from_ymd_opt(2023, 9, 1)
                    .unwrap()
                    .and_hms_milli_opt(15, 1, 38, 400)
                    .unwrap(),
                latest_replay_sha: "whatevs".to_string(),
                top_maps: vec!["Emerald City LE".to_string()],
            }
        );
    }
}
