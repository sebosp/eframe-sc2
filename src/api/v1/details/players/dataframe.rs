//! Polars queries for the map frequency

use super::{ListDetailsPlayerReq, ListDetailsPlayerRes, PlayerStats};
use crate::server::AppState;
use polars::prelude::*;

/// Gets the list of players from the details.ipc file
#[tracing::instrument(level = "debug", skip(state))]
pub async fn get_player_freq(
    req: ListDetailsPlayerReq,
    state: AppState,
) -> Result<ListDetailsPlayerRes, crate::error::Error> {
    let meta = crate::meta::ResponseMetaBuilder::new();
    let mut query = LazyFrame::scan_ipc(
        format!("{}/{}", state.source_dir, crate::DETAILS_IPC),
        Default::default(),
    )?
    .explode(["player_list"])
    .unnest(["player_list"])
    .with_columns(vec![
        col("toon"),
        col("name")
            .str()
            .split(lit("<sp/>"))
            .list()
            .last()
            .alias("player_name"),
    ])
    .filter(
        col("ext_datetime")
            .gt(lit(req.file_min_date))
            .and(col("ext_datetime").lt(lit(req.file_max_date))),
    );
    if !req.file_name.is_empty() {
        query = query.filter(
            col("ext_fs_replay_file_name")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.file_name.to_lowercase())),
        );
    }
    if !req.file_hash.is_empty() {
        query = query.filter(
            col("ext_fs_replay_sha256")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.file_hash.to_lowercase())),
        );
    }
    let map_players_freq = query
        .clone()
        .group_by([col("toon"), col("player_name")])
        .agg([col("title")
            .value_counts(true, true, "counts", true)
            .struct_()
            .field_by_index(0)
            .head(Some(5))
            .alias("top_maps")])
        .sort(
            ["player_name"],
            SortMultipleOptions {
                descending: vec![true],
                ..Default::default()
            },
        );
    if !req.name.is_empty() {
        query = query.filter(
            col("player_name")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.name.to_lowercase())),
        );
    } else {
        query = query.filter(col("player_name").str().contains_literal(lit("A.I")).not());
    }
    let latest_replay_shas = query
        .clone()
        .group_by([col("toon"), col("player_name")])
        .agg([col("ext_fs_replay_sha256")
            .last()
            .alias("latest_replay_sha")]);
    let per_race_stats = query
        .clone()
        .group_by([col("toon"), col("player_name"), col("result"), col("race")])
        .agg([col("result").count().alias("result_count")])
        .sort(
            ["result_count", "toon", "player_name"],
            SortMultipleOptions {
                descending: vec![true, true, true],
                ..Default::default()
            },
        )
        .collect()?;
    tracing::info!("PerRaceStats: {:?}", per_race_stats);
    let res = query
        .group_by([col("toon"), col("player_name")])
        .agg([
            col("player_name").count().alias("count"),
            col("ext_datetime")
                .min()
                .dt()
                .to_string("%Y-%m-%dT%H:%M:%S")
                .alias("min_date"),
            col("ext_datetime")
                .max()
                .dt()
                .to_string("%Y-%m-%dT%H:%M:%S")
                .alias("max_date"),
        ])
        .join(
            latest_replay_shas,
            &[col("toon"), col("player_name")],
            &[col("toon"), col("player_name")],
            JoinArgs::new(JoinType::Inner),
        )
        .join(
            map_players_freq,
            &[col("toon"), col("player_name")],
            &[col("toon"), col("player_name")],
            JoinArgs::new(JoinType::Inner),
        )
        /*.join(
            per_race_stats,
            &[col("toon"), col("player_name")],
            &[col("toon"), col("player_name")],
            JoinArgs::new(JoinType::Left),
        )*/
        .sort(
            ["count"],
            SortMultipleOptions {
                descending: vec![true],
                ..Default::default()
            },
        )
        .limit(10000) // TODO: Unhardcode
        .collect()?;
    tracing::trace!("ListDetailsPlayerRes: {:?}", res);
    let data_str = crate::common::convert_df_to_json_data(&res)?;
    tracing::trace!("Data: {}", data_str);
    let data: Vec<PlayerStats> = serde_json::from_str(&data_str)?;

    Ok(ListDetailsPlayerRes {
        meta: meta.build(),
        data,
    })
}
