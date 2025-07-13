//! Polars queries for the map frequency

use super::{ListDetailsMapReq, ListDetailsMapRes, MapStats};
use crate::server::AppState;
use polars::prelude::*;

/// Gets the list of maps from the details.ipc file
#[tracing::instrument(level = "debug", skip(state))]
pub async fn get_map_freq(
    req: ListDetailsMapReq,
    state: AppState,
) -> Result<ListDetailsMapRes, crate::error::Error> {
    let meta = crate::meta::ResponseMetaBuilder::new();
    let mut details_query = LazyFrame::scan_ipc(
        format!("{}/{}", state.source_dir, crate::DETAILS_IPC),
        Default::default(),
    )?
    .with_columns(vec![col("player_name")
        .str()
        .split(lit("<sp/>"))
        .list()
        .last()
        .alias("player_name")])
    .filter(
        col("ext_datetime")
            .gt(lit(req.file_min_date))
            .and(col("ext_datetime").lt(lit(req.file_max_date))),
    );
    if !req.title.is_empty() {
        details_query = details_query.filter(
            col("title")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.title.to_lowercase())),
        );
    }
    if !req.file_name.is_empty() {
        details_query = details_query.filter(
            col("ext_fs_replay_file_name")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.file_name.to_lowercase())),
        );
    }
    if !req.replay_id.is_empty() {
        details_query = details_query.filter(col("ext_fs_id").eq(lit(req.replay_id)));
    }
    let mut player1_match_query = details_query.clone();
    let mut player2_match_query = details_query.clone();
    let map_players_freq = details_query
        .clone()
        .group_by([col("title")])
        .agg([col("player_name")
            .value_counts(true, true, "counts", true)
            .struct_()
            .field_by_index(0)
            .head(Some(5))
            .alias("top_players")])
        .sort(
            ["title"],
            SortMultipleOptions {
                descending: vec![true],
                ..Default::default()
            },
        );
    if !req.player_1.is_empty() && req.player_2.is_empty() {
        details_query = details_query.filter(
            col("player_name")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.player_1.to_lowercase())),
        );
    } else if !req.player_1.is_empty() && !req.player_2.is_empty() {
        // If both players are specified, we need to create views
        // for each player and then join them later.
        player1_match_query =
            player1_match_query.filter(col("player_name").eq(lit(req.player_1.to_lowercase())));
        player2_match_query =
            player2_match_query.filter(col("player_name").eq(lit(req.player_2.to_lowercase())));
    }
    let latest_replay_ids = details_query
        .clone()
        .group_by([col("title")])
        .agg([col("ext_fs_id").last().alias("latest_replay_id")]);
    let res = tokio::task::spawn_blocking(move || {
        let mut query_plan = details_query;
        if !req.player_1.is_empty() && !req.player_2.is_empty() {
            player1_match_query = player1_match_query.select(&[col("ext_fs_id")]);
            player2_match_query = player2_match_query.select(&[col("ext_fs_id")]);
            player1_match_query = player1_match_query.select(&[col("ext_fs_id")]);
            query_plan = query_plan
                .join(
                    player1_match_query,
                    &[col("ext_fs_id")],
                    &[col("ext_fs_id")],
                    JoinArgs::new(JoinType::Inner),
                )
                .join(
                    player2_match_query,
                    &[col("ext_fs_id")],
                    &[col("ext_fs_id")],
                    JoinArgs::new(JoinType::Inner),
                );
        }
        query_plan
            .group_by([col("title")])
            .agg([
                col("title").count().alias("count"),
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
                latest_replay_ids,
                &[col("title")],
                &[col("title")],
                JoinArgs::new(JoinType::Inner),
            )
            .join(
                map_players_freq,
                &[col("title")],
                &[col("title")],
                JoinArgs::new(JoinType::Inner),
            )
            .sort(
                ["count"],
                SortMultipleOptions {
                    descending: vec![true],
                    ..Default::default()
                },
            )
            .limit(10000) // TODO: Unhardcode
            .collect()
    })
    .await
    .unwrap();
    let res = res?;
    tracing::trace!("ListDetailsMapRes: {:?}", res);
    let data_str = crate::common::convert_df_to_json_data(&res)?;
    tracing::trace!("Data: {}", data_str);
    let data: Vec<MapStats> = serde_json::from_str(&data_str)?;

    Ok(ListDetailsMapRes {
        meta: meta.build(),
        data,
    })
}
