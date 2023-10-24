//! Axum server module

use super::SnapshotStats;
use crate::meta::ResponseMeta;
use crate::server::AppState;
use axum::{extract::Query, extract::State, http::StatusCode, Json};
use std::sync::Arc;

pub fn route_analyzed_snapshot_meta(
    state: State<Arc<AppState>>,
) -> (StatusCode, Json<SnapshotStats>) {
    match super::dataframe::get_metadata(state).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => {
            tracing::error!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AnalyzedSnapshotMeta {
                    meta: ResponseMeta {
                        status: "error".to_string(),
                        message: e.to_string(),
                        total: 0,
                        snapshot_epoch: chrono::Utc::now().timestamp_millis() as u64,
                    },
                    data: vec![],
                }),
            )
        }
    }
}
