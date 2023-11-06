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
    let mut query = LazyFrame::scan_ipc(
        format!("{}/{}", state.source_dir, crate::DETAILS_IPC),
        Default::default(),
    )?
    .filter(
        col("ext_datetime")
            .gt(lit(req.file_min_date))
            .and(col("ext_datetime").lt(lit(req.file_max_date))),
    );
    if !req.title.is_empty() {
        tracing::info!("Not empty title: {}", req.title);
        query = query.filter(
            col("title")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.title.to_lowercase())),
        );
    }
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
    if !req.player.is_empty() {
        query = query
            .explode(["player_list"])
            .unnest(["player_list"])
            .filter(
                col("name")
                    .str()
                    .to_lowercase()
                    .str()
                    .contains_literal(lit(req.player.to_lowercase())),
            );
    }
    let latest_replay_shas =
        query
            .clone()
            .group_by([col("title")])
            .agg([col("ext_fs_replay_sha256")
                .last()
                .alias("latest_replay_sha")]);

    let res = query
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
            latest_replay_shas,
            &[col("title")],
            &[col("title")],
            JoinArgs::new(JoinType::Inner),
        )
        .sort(
            "count",
            SortOptions {
                descending: true,
                ..Default::default()
            },
        )
        .limit(1000)
        .collect()?;
    tracing::trace!("ListDetailsMapRes: {:?}", res);
    let data_str = crate::common::convert_df_to_json_data(&res)?;
    tracing::info!("Data: {}", data_str);
    let data: Vec<MapStats> = serde_json::from_str(&data_str)?;

    Ok(ListDetailsMapRes {
        meta: meta.build(),
        data,
    })
}
