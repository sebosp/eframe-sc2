//! Contains the UI for the player frequency table.

use crate::app::AppEvent;

use super::PlayerStats;
use super::{ListDetailsPlayerReq, SC2PlayerPicker};
use eframe::egui;
use egui::Ui;
use egui::Widget;
use egui::{Color32, RichText};
use egui_extras::{Column, DatePickerButton, TableBuilder};

impl SC2PlayerPicker {
    /// Builds a portion of the UI to be used for the Players table.
    fn table_div(&mut self, ui: &mut Ui, players: &[PlayerStats]) {
        ui.vertical(|ui| {
            ui.heading("Players");
            ui.separator();
            ui.allocate_ui(
                egui::Vec2::new(ui.available_width(), ui.available_height() * 0.5),
                |ui| {
                    self.table_inner(ui, players);
                },
            );
        });
    }

    /// Builds a table for egui with basic players information.
    fn table_inner(&mut self, ui: &mut Ui, players: &[PlayerStats]) {
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
        let player_1 = self.request.player_1.clone();
        let player_2 = self.request.player_2.clone();

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Row");
                });
                header.col(|ui| {
                    ui.strong("Frequency");
                });
                header.col(|ui| {
                    ui.strong("Player Name");
                });
                header.col(|ui| {
                    ui.strong("Blizzard.com Link");
                });
                header.col(|ui| {
                    ui.strong("Top 5 Maps for player");
                });
            })
            .body(|mut body| {
                let max_games_on_player = players.iter().map(|x| x.count).max().unwrap_or(0);
                for (idx, player) in players.iter().enumerate() {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        let map_ratio = player.count as f32 / max_games_on_player as f32;
                        row.col(|ui| {
                            if player_1 == player.name || player_2 == player.name {
                                ui.strong(idx.to_string());
                            } else if ui.button(idx.to_string()).clicked() {
                                if self.request.player_1.is_empty() {
                                    self.request.player_1 = player.name.clone();
                                } else {
                                    self.request.player_2 = player.name.clone();
                                }
                                ui.label(idx.to_string());
                            }
                        });
                        row.col(|ui| {
                            // Create a bar that has the size of the total games divided by the current player count
                            let bar = egui::ProgressBar::new(map_ratio)
                                .desired_width(ui.available_width())
                                .text(player.count.to_string());
                            ui.add(bar);
                        });
                        row.col(|ui| {
                            if player_1 == player.name || player_2 == player.name {
                                ui.strong(&player.name);
                            } else if ui
                                .button(RichText::new(&player.name).color(Color32::GREEN))
                                .clicked()
                            {
                                if self.request.player_1.is_empty() {
                                    self.request.player_1 = player.name.clone();
                                } else {
                                    self.request.player_2 = player.name.clone();
                                }
                                self.req_details_players();
                                ui.label(&player.name);
                            }
                        });
                        row.col(|ui| {
                            egui::Hyperlink::from_label_and_url(
                                player.blizzard_profile_link_title(),
                                player.blizzard_profile_link_href(),
                            )
                            .open_in_new_tab(true)
                            .ui(ui);
                        });
                        row.col(|ui| {
                            ui.label(player.top_maps.join(", "));
                        });
                    });
                }
            });
    }

    pub fn update(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        _tx: tokio::sync::mpsc::Sender<AppEvent>,
    ) {
        egui::Window::new("Player Selection")
            .default_width(480.0)
            .default_height(480.0)
            .open(is_open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Filters > ");
                    ui.label("Player: ");
                    if ui
                        .text_edit_singleline(&mut self.request.player_name_like)
                        .changed()
                    {
                        self.req_details_players();
                    }
                    ui.label("File Path: ");
                    if ui
                        .text_edit_singleline(&mut self.request.file_name)
                        .changed()
                    {
                        self.req_details_players();
                    }
                    ui.label("Snapshot Replay ID: ");
                    if ui
                        .text_edit_singleline(&mut self.request.replay_id)
                        .changed()
                    {
                        self.req_details_players();
                    }
                    ui.label("Min date: ");
                    if DatePickerButton::new(&mut self.request.file_min_date)
                        .id_salt("min_date")
                        .ui(ui)
                        .changed()
                    {
                        self.req_details_players();
                    }
                    ui.label("Max date: ");
                    if DatePickerButton::new(&mut self.request.file_max_date)
                        .id_salt("max_date")
                        .ui(ui)
                        .changed()
                    {
                        self.req_details_players();
                    }
                });
                let player_list: Vec<PlayerStats> = if let Some(player_list) = &self.player_list {
                    if let Some(player_list) = player_list.ready() {
                        player_list.data.clone()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };
                self.request.file_min_date = player_list
                    .iter()
                    .map(|x| x.min_date.date())
                    .min()
                    .unwrap_or(ListDetailsPlayerReq::default_min_date());
                self.request.file_max_date = player_list
                    .iter()
                    .map(|x| x.max_date.date())
                    .max()
                    .unwrap_or(ListDetailsPlayerReq::default_max_date());
                self.table_div(ui, &player_list);
            });
    }
}
