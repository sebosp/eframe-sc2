//! Contains the UI for the map frequency table.

use super::MapStats;
use super::{ListDetailsMapReq, ListDetailsMapRes, SC2MapPicker};
use eframe::egui;
use egui::Ui;
use egui::Widget;
use egui_extras::{Column, DatePickerButton, TableBuilder};
use urlencoding::encode;

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
        query_params.push(format!(
            "file_min_date={}",
            encode(&filters.file_min_date.to_string())
        ));
        query_params.push(format!(
            "file_max_date={}",
            encode(&filters.file_max_date.to_string())
        ));
        let query_url = format!("/api/v1/details/maps?{}", query_params.join("&"));
        ehttp::fetch_async(ehttp::Request::get(query_url))
            .await
            .map(|response| serde_json::from_slice(&response.bytes).unwrap_or_default())
            .unwrap_or_default()
    }

    /// Requests the async operation to get the details of the maps to the HTTP server.
    pub fn req_details_maps(&mut self) {
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
            .default_height(480.0)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Filters > ");
                    ui.label("Map: ");
                    if ui.text_edit_singleline(&mut self.request.title).changed() {
                        self.req_details_maps();
                    }
                    ui.label("Player: ");
                    if ui.text_edit_singleline(&mut self.request.player).changed() {
                        self.req_details_maps();
                    }
                    ui.label("File Path: ");
                    if ui
                        .text_edit_singleline(&mut self.request.file_name)
                        .changed()
                    {
                        self.req_details_maps();
                    }
                    ui.label("File Hash: ");
                    if ui
                        .text_edit_singleline(&mut self.request.file_hash)
                        .changed()
                    {
                        self.req_details_maps();
                    }
                    ui.label("Min date: ");
                    if DatePickerButton::new(&mut self.request.file_min_date)
                        .id_source("min_date")
                        .ui(ui)
                        .changed()
                    {
                        self.req_details_maps();
                    }
                    ui.label("Max date: ");
                    if DatePickerButton::new(&mut self.request.file_max_date)
                        .id_source("max_date")
                        .ui(ui)
                        .changed()
                    {
                        self.req_details_maps();
                    }
                });
                if let Some(response) = &self.response {
                    if let Some(response) = response.ready() {
                        table_div(ui, &response.data);
                    }
                }
            });
    }
}
/// Builds a portion of the UI to be used for the Maps table.
fn table_div(ui: &mut Ui, maps: &[MapStats]) {
    ui.vertical(|ui| {
        ui.heading("Maps");
        ui.separator();
        ui.allocate_ui(
            egui::Vec2::new(ui.available_width(), ui.available_height() * 0.5),
            |ui| {
                table_inner(ui, maps);
            },
        );
    });
}

/// Builds a table for egui with basic map information.
fn table_inner(ui: &mut Ui, maps: &[MapStats]) {
    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .column(Column::initial(100.0).at_least(40.0).clip(true))
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::remainder())
        .min_scrolled_height(0.0);

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Row");
            });
            header.col(|ui| {
                ui.strong("freq");
            });
            header.col(|ui| {
                ui.strong("title");
            });
        })
        .body(|mut body| {
            let max_games_on_map = maps.iter().map(|x| x.count).max().unwrap_or(0);
            for (idx, map) in maps.iter().enumerate() {
                let row_height = 18.0;
                body.row(row_height, |mut row| {
                    let map_ratio = map.count as f32 / max_games_on_map as f32;
                    row.col(|ui| {
                        ui.label(idx.to_string());
                    });
                    row.col(|ui| {
                        // Create a bar that has the size of the total games divided by the current map count
                        let bar = egui::ProgressBar::new(map_ratio)
                            .desired_width(ui.available_width())
                            .text(map.count.to_string());
                        ui.add(bar);
                    });
                    row.col(|ui| {
                        ui.label(map.title.clone());
                    });
                });
            }
        });
}
