use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{job::JobContext, track::TracksFilters},
    worker::jobs::remove_track::{remove_track, RemoveTrackParams},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAlbumParams {
    pub album_id: i64,
}

pub async fn remove_album(context: JobContext<RemoveAlbumParams>) -> Result<()> {
    let album = context
        .state
        .album_service
        .find(context.params.album_id)
        .await?;

    if album.stats.tracks_count > 0 {
        let tracks = context
            .state
            .track_service
            .find_many(
                Some(&TracksFilters {
                    album_id: Some(album.album.id),
                    artist_id: None,
                    has_lyrics: None,
                }),
                None,
            )
            .await?;

        for track in &tracks {
            remove_track(context.clone_with_params(RemoveTrackParams { track_id: track.id }))
                .await?;
        }
    }

    if let Some(cover_path) = album.album.cover_path {
        context
            .state
            .image_service
            .remove(&PathBuf::from(&cover_path))
            .await?;
    }

    context.state.album_service.remove(album.album.id).await?;

    Ok(())
}
