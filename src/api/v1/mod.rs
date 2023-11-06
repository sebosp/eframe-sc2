//! API V1 routes

pub mod details;
pub mod snapshot_stats;
pub mod tracker_events;

#[cfg(not(target_arch = "wasm32"))]
use axum::{extract::State, routing::get, Router};

#[cfg(not(target_arch = "wasm32"))]
use crate::server::AppState;

#[cfg(not(target_arch = "wasm32"))]
pub fn routes(state: State<AppState>) -> Router {
    Router::new()
        .route(
            "/snapshot_stats",
            get(snapshot_stats::server::route_snapshot_stats),
        )
        .with_state(state.0.clone())
        .nest("/details", details::routes(state))
}
