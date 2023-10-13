//! Details query API

use crate::common::*;
use crate::details::*;
use crate::server::AppState;
use axum::{extract::Query, extract::State, http::StatusCode, Json};
use polars::prelude::*;

/// Filters the available maps based on the query parameters
pub async fn axum_route_query_maps(
    req: Query<ListReplayReq>,
    state: State<Arc<AppState>>,
) -> (StatusCode, Json<ListReplayRes>) {
    match polars_get_map_freq(req, state).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListReplayRes {
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
pub async fn polars_get_map_freq(
    req: Query<ListReplayReq>,
    state: State<Arc<AppState>>,
) -> Result<ListReplayRes, crate::error::Error> {
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

    Ok(ListReplayRes {
        meta: ResponseMeta {
            status: "ok".to_string(),
            total: data.len(),
            snapshot_epoch: chrono::Utc::now().timestamp_millis() as u64,
            message: "".to_string(),
        },
        data,
    })
}
