//! Axum route handlers

use super::{ListDetailsMapFreqReq, ListDetailsMapFreqRes};
use crate::meta::ResponseMeta;
use crate::server::AppState;
use axum::{extract::Query, extract::State, http::StatusCode, routing::get, Json, Router};
use std::sync::Arc;

pub fn routes(state: State<Arc<AppState>>) -> Router {
    Router::new()
        .route("/map_frequency", get(route_query_maps))
        .with_state(state.0)
}

/// Filters the available maps based on the query parameters
pub async fn route_query_maps(
    req: Query<ListDetailsMapFreqReq>,
    state: State<Arc<AppState>>,
) -> (StatusCode, Json<ListDetailsMapFreqRes>) {
    match super::dataframe::get_map_freq(req, state).await {
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
