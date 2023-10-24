//! Polars queries for the map frequency

use super::{ListDetailsMapReq, ListDetailsMapRes, MapCount};
use crate::server::AppState;
use polars::prelude::*;

/// Gets the list of maps from the details.ipc file
pub async fn get_map_freq(
    req: ListDetailsMapReq,
    state: AppState,
) -> Result<ListDetailsMapRes, crate::error::Error> {
    let meta = crate::meta::ResponseMetaBuilder::new();
    let mut query = LazyFrame::scan_ipc(
        format!("{}/{}", state.source_dir, crate::DETAILS_IPC),
        Default::default(),
    )?;
    if !req.title.is_empty() {
        query = query.filter(
            col("title")
                .str()
                .to_lowercase()
                .str()
                .contains_literal(lit(req.title.to_lowercase())),
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
    let res = query
        .group_by([col("title")])
        .agg([
            col("title").count().alias("count"),
            col("ext_datetime").min().alias("min_date"),
            col("ext_datetime").max().alias("max_date"),
        ])
        .sort(
            "count",
            SortOptions {
                descending: true,
                ..Default::default()
            },
        )
        .limit(1000)
        .collect()?;
    let num_maps = res.height();
    let data_str = crate::common::convert_df_to_json_data(&res)?;
    let data: Vec<MapCount> = serde_json::from_str(&data_str)?;

    Ok(ListDetailsMapRes {
        meta: meta.build(),
        data,
    })
}
