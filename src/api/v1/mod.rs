//! API V1 routes

pub mod analyzed_snapshot_meta;
pub mod details;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use axum::{extract::State, routing::get, Router};

#[cfg(not(target_arch = "wasm32"))]
use crate::server::AppState;

#[cfg(not(target_arch = "wasm32"))]
pub fn routes(state: State<Arc<AppState>>) -> Router {
    Router::new()
        .route(
            "/analyzed_snapshot_meta",
            get(analyzed_snapshot_meta::server::route_analyzed_snapshot_meta),
        )
        .with_state(state)
        .nest("/details", details::routes(state))
}
