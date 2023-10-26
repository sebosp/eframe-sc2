//! API versions

pub mod v1;

#[cfg(not(target_arch = "wasm32"))]
use crate::server::AppState;

#[cfg(not(target_arch = "wasm32"))]
use axum::{extract::State, Router};

#[cfg(not(target_arch = "wasm32"))]
pub fn routes(shared_state: State<AppState>) -> Router {
    Router::new().nest("/v1", v1::routes(shared_state))
}
