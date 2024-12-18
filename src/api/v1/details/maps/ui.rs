//! Contains the UI for the map frequency table.

use crate::app::AppEvent;

use super::MapStats;
use super::{ListDetailsMapReq, SC2MapPicker};
use eframe::egui;
use egui::Ui;
use egui::Widget;
use egui_extras::{Column, DatePickerButton, TableBuilder};

impl SC2MapPicker {
    /// Builds a portion of the UI to be used for the Maps table.
    fn table_div(&mut self, ui: &mut Ui, maps: &[MapStats]) {
        ui.vertical(|ui| {
            ui.heading("Maps");
            ui.separator();
            ui.allocate_ui(
                egui::Vec2::new(ui.available_width(), ui.available_height() * 0.5),
                |ui| {
                    self.table_inner(ui, maps);
                },
            );
        });
    }

    /// Builds a table for egui with basic map information.
    fn table_inner(&mut self, ui: &mut Ui, maps: &[MapStats]) {
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::initial(100.0).at_least(40.0).clip(true))
            .column(Column::initial(80.0).at_least(40.0).clip(true))
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0);
        let selected_map_title: String = if let Some(selected_map) = &self.selected_map {
            selected_map.title.clone()
        } else {
            "".to_string()
        };

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Row");
                });
                header.col(|ui| {
                    ui.strong("Frequency");
                });
                header.col(|ui| {
                    ui.strong("Map Title");
                });
                header.col(|ui| {
                    ui.strong("Liquipedia Link");
                });
                header.col(|ui| {
                    ui.strong("Top 5 Players on Map");
                });
            })
            .body(|mut body| {
                let max_games_on_map = maps.iter().map(|x| x.count).max().unwrap_or(0);
                for (idx, map) in maps.iter().enumerate() {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        let map_ratio = map.count as f32 / max_games_on_map as f32;
                        row.col(|ui| {
                            if selected_map_title == map.title {
                                ui.strong(idx.to_string());
                            } else if ui.button(idx.to_string()).clicked() {
                                self.selected_map = Some(map.clone());
                                ui.label(idx.to_string());
                            }
                        });
                        row.col(|ui| {
                            // Create a bar that has the size of the total games divided by the current map count
                            let bar = egui::ProgressBar::new(map_ratio)
                                .desired_width(ui.available_width())
                                .text(map.count.to_string());
                            ui.add(bar);
                        });
                        row.col(|ui| {
                            if selected_map_title == map.title {
                                ui.strong(&map.title);
                            } else if ui.button(&map.title).clicked() {
                                self.selected_map = Some(map.clone());
                                ui.label(&map.title);
                            }
                        });
                        row.col(|ui| {
                            egui::Hyperlink::from_label_and_url(
                                map.clean_map_title(),
                                map.liquipedia_map_link(),
                            )
                            .open_in_new_tab(true)
                            .ui(ui);
                        });
                        row.col(|ui| {
                            ui.label(map.top_players.join(", "));
                        });
                    });
                }
            });
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        tx: tokio::sync::mpsc::Sender<AppEvent>,
    ) {
        egui::Window::new("Map Selection")
            .default_width(480.0)
            .default_height(480.0)
            .open(is_open)
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
                let map_list: Vec<MapStats> = if let Some(map_list) = &self.map_list {
                    if let Some(map_list) = map_list.ready() {
                        map_list.data.clone()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };
                self.request.file_min_date = map_list
                    .iter()
                    .map(|x| x.min_date.date())
                    .min()
                    .unwrap_or(ListDetailsMapReq::default_min_date());
                self.request.file_max_date = map_list
                    .iter()
                    .map(|x| x.max_date.date())
                    .max()
                    .unwrap_or(ListDetailsMapReq::default_max_date());
                self.table_div(ui, &map_list);
            });
    }
}
