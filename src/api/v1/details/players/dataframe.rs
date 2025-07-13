//! Polars queries for the map frequency

use super::{ListDetailsPlayerReq, ListDetailsPlayerRes, PlayerStats};
use crate::server::AppState;
use polars::prelude::*;

/// Shortens the player name to remove the clan tag.
fn player_name_xfrm() -> Expr {
    col("player_name")
        .str()
        .split(lit("<sp/>"))
        .list()
        .last()
        .alias("player_name")
}

/// Group_by used by most queries in this file.
fn group_by_player() -> Vec<Expr> {
    vec![
        col("player_toon_region"),
        col("player_toon_program_id"),
        col("player_toon_realm"),
        col("player_toon_id"),
        col("player_name"),
    ]
}

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
    .with_columns(vec![
        col("player_toon_region"),
        col("player_toon_program_id"),
        col("player_toon_realm"),
        col("player_toon_id"),
        col("ext_fs_id"),
        player_name_xfrm(),
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
    if !req.map_title.is_empty() {
        query = query.filter(col("title").eq(lit(req.map_title)));
    }
    if !req.replay_id.is_empty() {
        query = query.filter(col("ext_fs_id").eq(lit(req.replay_id)));
    }
    let mut map_players_freq = query.clone();
    let mut matches_with_desired_player = query.clone();
    let player_filter = if !req.player_1.is_empty() {
        req.player_1.clone()
    } else {
        req.player_2.clone()
    };

    if !player_filter.is_empty() {
        map_players_freq =
            map_players_freq.filter(col("player_name").eq(lit(player_filter.clone())).not());
        //query = query.filter(col("player_name").eq(lit(req.player.clone()).not()));
        matches_with_desired_player = matches_with_desired_player
            .filter(col("player_name").eq(lit(player_filter.clone())))
            .select([col("ext_fs_id")]);
    }
    map_players_freq = map_players_freq
        .group_by(group_by_player())
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
    if !req.player_name_like.is_empty() {
        query = query.filter(
            col("player_name")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.player_name_like.to_lowercase())),
        );
    } else {
        query = query.filter(
            col("player_name")
                .str()
                .contains_literal(lit("A.I"))
                .not()
                .and(
                    col("player_name")
                        .str()
                        .contains_literal(lit("Cheater "))
                        .not(),
                ),
        );
    }
    let mut latest_replay_ids = query.clone();
    if !player_filter.is_empty() {
        latest_replay_ids =
            latest_replay_ids.filter(col("player_name").eq(lit(player_filter.clone())).not());
    }
    latest_replay_ids = latest_replay_ids
        .group_by(group_by_player())
        .agg([col("ext_fs_id").last().alias("latest_replay_id")]);

    let res = tokio::task::spawn_blocking(move || {
        let mut main_query = query;
        if !player_filter.is_empty() {
            matches_with_desired_player = matches_with_desired_player
                .group_by([col("ext_fs_id")])
                .agg([col("ext_fs_id")
                    .filter(col("ext_fs_id").count().eq(lit(2)))
                    .alias("ext_fs_id_count")]); // 1v1 only for now.
            main_query = main_query.join(
                matches_with_desired_player,
                &[col("ext_fs_id")],
                &[col("ext_fs_id")],
                JoinArgs::new(JoinType::Inner),
            );
        }
        main_query = main_query
            .group_by(group_by_player())
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
                latest_replay_ids,
                &group_by_player(),
                &group_by_player(),
                JoinArgs::new(JoinType::Inner),
            )
            .join(
                map_players_freq,
                &group_by_player(),
                &group_by_player(),
                JoinArgs::new(JoinType::Inner),
            );
        /*            .join(
            per_race_stats,
            &[
                col("player_toon_region"),
                col("player_toon_program_id"),
                col("player_toon_realm"),
                col("player_toon_id"),
                col("player_name"),
            ],
            &[
                col("player_toon_region"),
                col("player_toon_program_id"),
                col("player_toon_realm"),
                col("player_toon_id"),
                col("player_name"),
            ],
            JoinArgs::new(JoinType::Left),
        )*/
        let query_res = main_query
            .sort(
                ["count"],
                SortMultipleOptions {
                    descending: vec![true],
                    ..Default::default()
                },
            )
            .limit(1000) // TODO: Unhardcode
            .collect()
            .expect("Failed to collect DataFrame");
        let data_str = crate::common::convert_df_to_json_data(&query_res)
            .expect("Failed to convert DataFrame to JSON data");
        let data: Vec<PlayerStats> = serde_json::from_str(&data_str)
            .expect("Failed to deserialize DataFrame to PlayerStats");
        data
    })
    .await;
    tracing::trace!("ListDetailsPlayerRes: {:?}", res);

    Ok(ListDetailsPlayerRes {
        meta: meta.build(),
        data: res.unwrap_or_default(),
    })
}
