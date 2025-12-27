use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        job::JobContext,
        track::{Track, TracksFilters},
    },
    worker::jobs::search_track::{search_track, SearchTrackParams},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAlbumParams {
    pub album_id: i64,
}

pub async fn search_album(context: JobContext<SearchAlbumParams>) -> Result<()> {
    let tracks = context
        .state
        .track_service
        .find_many(
            Some(&TracksFilters {
                album_id: Some(context.params.album_id),
                artist_id: None,
            }),
            None,
        )
        .await?;

    let filtered: Vec<Track> = tracks.into_iter().filter(|t| !t.has_lyrics).collect();

    for (i, track) in filtered.iter().enumerate() {
        context.log(format!(
            "[{}/{}] Searching track: {}",
            i + 1,
            filtered.len(),
            track.title
        ));
        search_track(context.clone_with_params(SearchTrackParams { track_id: track.id })).await?;
    }

    Ok(())
}
