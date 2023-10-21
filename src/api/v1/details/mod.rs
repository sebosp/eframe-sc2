//! Details query API

pub mod map_frequency;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use axum::{extract::State, Router};

#[cfg(not(target_arch = "wasm32"))]
use crate::server::AppState;

#[cfg(not(target_arch = "wasm32"))]
pub fn routes(state: State<Arc<AppState>>) -> Router {
    Router::new().nest("/map_frequency", map_frequency::server::routes(state))
}
