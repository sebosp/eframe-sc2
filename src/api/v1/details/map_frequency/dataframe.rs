//! Polars queries for the map frequency

use super::{ListDetailsMapFreqReq, ListDetailsMapFreqRes};
use crate::api::v1::details::map_frequency::MapFrequency;
use crate::server::AppState;
use axum::{extract::Query, extract::State};
use polars::prelude::*;
use std::sync::Arc;

/// Gets the list of maps from the details.ipc file
pub async fn get_map_freq(
    req: Query<ListDetailsMapFreqReq>,
    state: State<Arc<AppState>>,
) -> Result<ListDetailsMapFreqRes, crate::error::Error> {
    let mut query = LazyFrame::scan_ipc(
        format!("{}/{}", state.source_dir, crate::DETAILS_IPC),
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
    let data_str = crate::common::convert_df_to_json_data(&res)?;
    let data: Vec<MapFrequency> = serde_json::from_str(&data_str)?;

    Ok(ListDetailsMapFreqRes {
        meta: crate::meta::ResponseMeta {
            status: "ok".to_string(),
            total: data.len(),
            snapshot_epoch: chrono::Utc::now().timestamp_millis() as u64,
            message: "".to_string(),
        },
        data,
    })
}
