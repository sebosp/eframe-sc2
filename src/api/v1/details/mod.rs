//! Details query API

pub mod maps;

#[cfg(not(target_arch = "wasm32"))]
use crate::server::AppState;

#[cfg(not(target_arch = "wasm32"))]
use axum::{extract::State, routing::get, Router};

#[cfg(not(target_arch = "wasm32"))]
pub fn routes(state: State<AppState>) -> Router {
    Router::new()
        .route("/maps", get(maps::server::route_query_maps))
        .with_state(state.0)
}
