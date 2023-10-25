//! Map count related queries

use urlencoding::encode;
pub mod ui;

#[cfg(not(target_arch = "wasm32"))]
pub mod dataframe;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

use eframe::egui;
use serde::{Deserialize, Serialize};

/// Basic query request available for filtering replay maps
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
    pub file_min_date: String,
    /// Max bound of the file date
    #[serde(default)]
    pub file_max_date: String,
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
    /// The number of players
    pub num_players: usize,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct SC2MapPicker {
    /// A set of filters for the maps
    #[serde(skip)]
    request: ListDetailsMapReq,

    /// Contains the metadata related to the backend snapshot.
    #[serde(skip)]
    response: Option<poll_promise::Promise<ListDetailsMapRes>>,

    /// Wether the map selection is open
    #[serde(skip)]
    pub is_open_map_selection: bool,
}

impl SC2MapPicker {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    async fn get_details_maps(filters: ListDetailsMapReq) -> ListDetailsMapRes {
        let mut query_params: Vec<String> = vec![];
        query_params.push(format!("title={}", encode(&filters.title)));
        query_params.push(format!("player={}", encode(&filters.player)));
        query_params.push(format!("file_name={}", encode(&filters.file_name)));
        query_params.push(format!("file_hash={}", encode(&filters.file_hash)));
        query_params.push(format!("file_min_date={}", encode(&filters.file_min_date)));
        query_params.push(format!("file_max_date={}", encode(&filters.file_max_date)));
        let query_url = format!("/api/v1/details/maps?{}", query_params.join("&"));
        ehttp::fetch_async(ehttp::Request::get(query_url))
            .await
            .map(|response| serde_json::from_slice(&response.bytes).unwrap_or_default())
            .unwrap_or_default()
    }

    /// Requests the async operation to get the details of the maps to the HTTP server.
    fn req_details_maps(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.response = Some(poll_promise::Promise::spawn_local(Self::get_details_maps(
                self.request.clone(),
            )));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.response = Some(poll_promise::Promise::spawn_async(Self::get_details_maps(
                self.request.clone(),
            )));
        }
    }
}

impl eframe::App for SC2MapPicker {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut open = self.is_open_map_selection;
        egui::Window::new("Map Selection")
            .default_width(480.0)
            .default_height(320.0)
            .open(&mut open)
            .show(ctx, |ui| {
                if ui.button("Load Backend Stats").clicked() {
                    self.req_details_maps();
                }
                ui.horizontal(|ui| {
                    ui.label("Filters > ");
                    ui.label("Maps Title: ");
                    ui.text_edit_singleline(&mut self.request.title);
                });
                if let Some(response) = &self.response {
                    if let Some(response) = response.ready() {
                        ui::table_div(ui, &response.data);
                    }
                }
            });
    }
}
