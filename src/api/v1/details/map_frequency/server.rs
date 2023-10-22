//! Axum route handlers

use super::{ListDetailsMapReq, ListDetailsMapRes};
use crate::meta::ResponseMeta;
use crate::server::AppState;
use axum::{extract::Query, extract::State, http::StatusCode, Json};
use std::sync::Arc;

/// Filters the available maps based on the query parameters
pub async fn route_query_maps(
    req: Query<ListDetailsMapReq>,
    state: State<Arc<AppState>>,
) -> (StatusCode, Json<ListDetailsMapRes>) {
    match super::dataframe::get_map_freq(req, state).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListDetailsMapRes {
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
