//! Polars queries for the map frequency

use super::{ListDetailsMapReq, ListDetailsMapRes, MapCount};
use crate::server::AppState;
use axum::{extract::Query, extract::State};
use polars::prelude::*;
use std::sync::Arc;

/// Gets the list of maps from the details.ipc file
pub async fn get_map_freq(
    req: Query<ListDetailsMapReq>,
    state: State<Arc<AppState>>,
) -> Result<ListDetailsMapRes, crate::error::Error> {
    let mut query = LazyFrame::scan_ipc(
        format!("{}/{}", state.source_dir, crate::DETAILS_IPC),
        Default::default(),
    )?;
    if !req.title.is_empty() {
        query = query.filter(
            col("title")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.title.to_lowercase())),
        );
    }
    if !req.player.is_empty() {
        query = query
            .explode(["player_list"])
            .unnest(["player_list"])
            .filter(
                col("name")
                    .str()
                    .to_lowercase()
                    .str()
                    .contains_literal(lit(req.player.to_lowercase())),
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
    let data_str = crate::common::convert_df_to_json_data(&res)?;
    let data: Vec<MapCount> = serde_json::from_str(&data_str)?;

    Ok(ListDetailsMapRes {
        meta: crate::meta::ResponseMeta {
            status: "ok".to_string(),
            total: data.len(),
            snapshot_epoch: chrono::Utc::now().timestamp_millis() as u64,
            message: "".to_string(),
        },
        data,
    })
}
