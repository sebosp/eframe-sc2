//! API V1 routes

pub mod details;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use axum::{extract::State, Router};

#[cfg(not(target_arch = "wasm32"))]
use crate::server::AppState;

#[cfg(not(target_arch = "wasm32"))]
pub fn routes(state: State<Arc<AppState>>) -> Router {
    Router::new().nest("/details", details::routes(state))
}
