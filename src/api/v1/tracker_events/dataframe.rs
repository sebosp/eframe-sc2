//! Polars queries for the unit born position Events

use super::{UnitBornPosEvent, UnitBornPosReq, UnitBornPosRes};
use crate::server::AppState;
use polars::prelude::*;

/// Gets the list of maps from the details.ipc file
#[tracing::instrument(level = "debug", skip(state))]
pub async fn get_map_freq(
    req: UnitBornPosReq,
    state: AppState,
) -> Result<UnitBornPosRes, crate::error::Error> {
    let meta = crate::meta::ResponseMetaBuilder::new();
    let mut query = LazyFrame::scan_ipc(
        format!("{}/{}", state.source_dir, crate::UNIT_BORN_IPC),
        Default::default(),
    )?;
    if !req.file_hash.is_empty() {
        query = query.filter(
            col("ext_fs_replay_sha256")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.file_hash.to_lowercase())),
        );
    }
    if let Some(player_name) = req.player {
        query = query.filter(
            col("ext_replay_detail_player_name")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(player_name.to_lowercase())),
        );
    } else {
        // This would be for a neutral unit.
        query = query.filter(
            col("ext_replay_detail_player_name")
                .str()
                .to_lowercase()
                .eq(lit("")),
        );
    }
    if !req.unit_type_name.is_empty() {
        query = query.filter(
            col("unit_type_name")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.unit_type_name.to_lowercase())),
        );
    }
    if let Some(game_loop) = req.game_loop {
        query = query.filter(col("ext_replay_loop").eq(lit(game_loop)));
    }
    let res = query
        .select(&[
            col("unit_type_name"),
            col("x"),
            col("y"),
            col("ext_replay_loop"),
        ])
        .sort(
            ["ext_replay_loop"],
            SortMultipleOptions {
                descending: vec![true],
                ..Default::default()
            },
        )
        .limit(1000)
        .collect()?;
    let data_str = crate::common::convert_df_to_json_data(&res)?;
    tracing::info!("Data: {}", data_str);
    let data: Vec<UnitBornPosEvent> = serde_json::from_str(&data_str)?;

    Ok(UnitBornPosRes {
        meta: meta.build(),
        data,
    })
}
