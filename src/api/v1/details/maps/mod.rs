//! Map count related queries

pub mod ui;

#[cfg(not(target_arch = "wasm32"))]
pub mod dataframe;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

use serde::{Deserialize, Serialize};

/// Basic query request available for filtering replay maps
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListDetailsMapReq {
    /// The title of the map
    #[serde(default)]
    pub title: String,
    /// A player that must have played in the game
    #[serde(default)]
    pub player: String,
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

impl Default for ListDetailsMapReq {
    fn default() -> Self {
        Self {
            title: Default::default(),
            player: Default::default(),
            file_name: Default::default(),
            file_hash: Default::default(),
            file_min_date: Self::default_min_date(),
            file_max_date: Self::default_max_date(),
        }
    }
}

impl ListDetailsMapReq {
    /// Returns a new instance of the request with the unescaped values
    pub fn from_escaped(self) -> Self {
        Self {
            title: urlencoding::decode(&self.title)
                .unwrap_or_default()
                .to_string(),
            player: urlencoding::decode(&self.player)
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

/// Basic query response available for filtering replay maps
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ListDetailsMapRes {
    /// Metadata of the response
    pub meta: crate::meta::ResponseMeta,
    /// The data of the response
    pub data: Vec<MapStats>,
}

/// Basic response for map frequency
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MapStats {
    /// Teh name of the map
    pub title: String,
    /// The amount of replays on this map
    pub count: u32,
    /// The minimum date of the snapshot taken
    pub min_date: chrono::NaiveDateTime,
    /// The maximum date of the snapshot taken
    pub max_date: chrono::NaiveDateTime,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct SC2MapPicker {
    /// A set of filters for the maps
    #[serde(skip)]
    request: ListDetailsMapReq,

    /// Contains the metadata related to the backend snapshot.
    #[serde(skip)]
    map_list: Option<poll_promise::Promise<ListDetailsMapRes>>,

    /// Wether the map selection is open
    #[serde(skip)]
    pub is_open_map_selection: bool,

    /// The selected map
    #[serde(skip)]
    pub selected_map: Option<String>,
}

// test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_map_stats() {
        let example_str = r#"{"title":"Emerald City LE","count":1852,"min_date":"2021-04-12T13:55:57.058","max_date":"2023-09-01T15:01:38.400","num_players":1852}"#;
        let example: MapStats = serde_json::from_str(example_str).unwrap();
        assert_eq!(
            example,
            MapStats {
                title: "Emerald City LE".to_string(),
                count: 1852,
                min_date: chrono::NaiveDate::from_ymd_opt(2021, 4, 12)
                    .unwrap()
                    .and_hms_milli_opt(13, 55, 57, 58)
                    .unwrap(),
                max_date: chrono::NaiveDate::from_ymd_opt(2023, 9, 1)
                    .unwrap()
                    .and_hms_milli_opt(15, 1, 38, 400)
                    .unwrap(),
            }
        );
    }
}
