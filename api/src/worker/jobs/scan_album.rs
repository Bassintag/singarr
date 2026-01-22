use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{job::JobContext, track::TracksFilters},
    worker::jobs::{
        clean_album::{clean_album, CleanAlbumParams},
        scan_track::{scan_track, ScanTrackParams},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanAlbumParams {
    pub album_id: i64,
}

pub async fn scan_album(context: JobContext<ScanAlbumParams>) -> Result<()> {
    let tracks = context
        .state
        .track_service
        .find_many(
            Some(&TracksFilters {
                album_id: Some(context.params.album_id),
                artist_id: None,
                has_lyrics: None,
            }),
            None,
        )
        .await?;

    for (i, track) in tracks.iter().enumerate() {
        context.log(format!(
            "[{}/{}] Scanning track: {}",
            i + 1,
            tracks.len(),
            track.title
        ));
        scan_track(context.clone_with_params(ScanTrackParams { track_id: track.id })).await?;
    }

    context.log("Cleaning removed tracks");

    clean_album(context.clone_with_params(CleanAlbumParams {
        album_id: context.params.album_id,
    }))
    .await?;

    Ok(())
}
