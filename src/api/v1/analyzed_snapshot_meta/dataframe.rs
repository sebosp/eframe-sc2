//! Dataframe module

use super::SnapshotStats;
use crate::meta::ResponseMetaBuilder;
use crate::server::AppState;

/// Gets the list of maps from the details.ipc file
pub async fn get_metadata(state: AppState) -> Result<SnapshotStats, crate::error::Error> {
    let meta = ResponseMetaBuilder::new();
    // Add the size of all the files in state.source_dir
    let mut directory_size = 0;
    for entry in std::fs::read_dir(&state.source_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();
        directory_size += size;
    }
    // get the date_modified of the details.ipc file
    let details_ipc_filename = format!("{}/{}", state.source_dir, crate::DETAILS_IPC);
    let date_modified = std::fs::metadata(&details_ipc_filename)?.modified()?;
    Ok(SnapshotStats {
        directory_size,
        date_modified,
        meta: meta.build(),
    })
}
