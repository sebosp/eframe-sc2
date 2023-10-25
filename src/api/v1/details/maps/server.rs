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
    let meta = ResponseMetaBuilder::new();
    match super::dataframe::get_map_freq(req.into(), state.into()).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListDetailsMapRes {
                    meta: meta
                        .with_status("error")
                        .with_message(e.to_string())
                        .build(),
                    data: vec![],
                }),
            )
        }
    }
}
