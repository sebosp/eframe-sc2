//! Details query API

pub mod map_frequency;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use crate::server::AppState;

#[cfg(not(target_arch = "wasm32"))]
use axum::{extract::State, routing::get, Router};

#[cfg(not(target_arch = "wasm32"))]
pub fn routes(state: State<Arc<AppState>>) -> Router {
    Router::new()
        .route(
            "/map_frequency",
            get(map_frequency::server::route_query_maps),
        )
        .with_state(state.0)
}
