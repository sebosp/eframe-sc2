//! Contains the UI for the map frequency table.

use super::MapStats;
use egui::Ui;
use egui_extras::{Column, TableBuilder};

/// Builds a portion of the UI to be used for the Maps table.
pub fn table_div(ui: &mut Ui, maps: &[MapStats]) {
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
pub fn table_inner(ui: &mut Ui, maps: &[MapStats]) {
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
