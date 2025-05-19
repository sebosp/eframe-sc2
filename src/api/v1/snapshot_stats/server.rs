//! Axum server module

use super::SnapshotStats;
use crate::meta::ResponseMetaBuilder;
use crate::server::AppState;
use axum::{extract::State, http::StatusCode, Json};
use std::time::SystemTime;

pub async fn route_snapshot_stats(
    State(state): State<AppState>,
) -> (StatusCode, Json<SnapshotStats>) {
    let meta = ResponseMetaBuilder::new();
    match super::dataframe::get_metadata(state).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SnapshotStats {
                    meta: meta.with_error(e.to_string()).build(),
                    directory_size: 0,
                    date_modified: SystemTime::UNIX_EPOCH,
                    directory: String::new(),
                }),
            )
        }
    }
}
