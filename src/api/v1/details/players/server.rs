//! Axum route handlers

use super::{ListDetailsPlayerReq, ListDetailsPlayerRes};
use crate::meta::ResponseMetaBuilder;
use crate::server::AppState;
use axum::{extract::Query, extract::State, http::StatusCode, Json};

/// Filters the available players based on the query parameters
pub async fn route_query_players(
    req: Query<ListDetailsPlayerReq>,
    State(state): State<AppState>,
) -> (StatusCode, Json<ListDetailsPlayerRes>) {
    tracing::info!("Querying Players: {:?}", req);
    let unescaped = req.0.from_escaped();
    let meta = ResponseMetaBuilder::new();
    match super::dataframe::get_player_freq(unescaped, state).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListDetailsPlayerRes {
                    meta: meta.with_error(e.to_string()).build(),
                    data: vec![],
                }),
            )
        }
    }
}
