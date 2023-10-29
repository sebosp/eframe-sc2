//! Axum route handlers

use super::{ListDetailsMapReq, ListDetailsMapRes};
use crate::meta::ResponseMetaBuilder;
use crate::server::AppState;
use axum::{extract::Query, extract::State, http::StatusCode, Json};

/// Filters the available maps based on the query parameters
pub async fn route_query_maps(
    req: Query<ListDetailsMapReq>,
    State(state): State<AppState>,
) -> (StatusCode, Json<ListDetailsMapRes>) {
    tracing::info!("Querying maps: {:?}", req);
    let unescaped = req.0.from_escaped();
    let meta = ResponseMetaBuilder::new();
    match super::dataframe::get_map_freq(unescaped, state).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListDetailsMapRes {
                    meta: meta.with_error(e.to_string()).build(),
                    data: vec![],
                }),
            )
        }
    }
}
