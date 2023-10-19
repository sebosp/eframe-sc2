//! Details query API

use crate::common::*;
use crate::details::*;
use egui::Ui;

#[cfg(not(target_arch = "wasm32"))]
use crate::server::AppState;

#[cfg(not(target_arch = "wasm32"))]
use axum::{extract::Query, extract::State, http::StatusCode, Json};

#[cfg(not(target_arch = "wasm32"))]
use polars::prelude::*;

/// Filters the available maps based on the query parameters
#[cfg(not(target_arch = "wasm32"))]
pub async fn axum_route_query_maps(
    req: Query<ListDetailsMapFreqReq>,
    state: State<Arc<AppState>>,
) -> (StatusCode, Json<ListDetailsMapFreqRes>) {
    match polars_get_map_freq(req, state).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListDetailsMapFreqRes {
                    meta: ResponseMeta {
                        status: "error".to_string(),
                        message: e.to_string(),
                        total: 0,
                        snapshot_epoch: chrono::Utc::now().timestamp_millis() as u64,
                    },
                    data: vec![],
                }),
            )
        }
    }
}

/// Gets the list of maps from the details.ipc file
#[cfg(not(target_arch = "wasm32"))]
pub async fn polars_get_map_freq(
    req: Query<ListDetailsMapFreqReq>,
    state: State<Arc<AppState>>,
) -> Result<ListDetailsMapFreqRes, crate::error::Error> {
    let mut query = LazyFrame::scan_ipc(
        format!("{}/{}", state.source_dir, DETAILS_IPC),
        Default::default(),
    )?;
    if let Some(title) = &req.title {
        query = query.filter(
            col("title")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(title.to_lowercase())),
        );
    }
    if let Some(player) = &req.player {
        query = query
            .explode(["player_list"])
            .unnest(["player_list"])
            .filter(
                col("name")
                    .str()
                    .to_lowercase()
                    .str()
                    .contains_literal(lit(player.to_lowercase())),
            );
    }
    let res = query
        .group_by([col("title")])
        .agg([col("title").count().alias("count")])
        .sort(
            "count",
            SortOptions {
                descending: true,
                ..Default::default()
            },
        )
        .limit(1000)
        .collect()?;
    let data_str = convert_df_to_json_data(&res)?;
    let data: Vec<MapFrequency> = serde_json::from_str(&data_str)?;

    Ok(ListDetailsMapFreqRes {
        meta: ResponseMeta {
            status: "ok".to_string(),
            total: data.len(),
            snapshot_epoch: chrono::Utc::now().timestamp_millis() as u64,
            message: "".to_string(),
        },
        data,
    })
}

/// Builds a table for egui with basic map information.
pub fn table_ui(ui: &mut Ui, maps: &[MapFrequency]) {
    use egui_extras::{Column, TableBuilder};

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
            header.col(|ui| {
                ui.strong("count");
            });
            header.col(|ui| {
                ui.strong("time activity");
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
                        let bar =
                            egui::ProgressBar::new(map_ratio).desired_width(ui.available_width());
                        ui.add(bar);
                    });
                    row.col(|ui| {
                        ui.label(map.title.clone());
                    });
                    row.col(|ui| {
                        ui.label(map.count.to_string());
                    });
                    row.col(|ui| {
                        ui.label("TODO: map activity");
                    });
                });
            }
        });
}
