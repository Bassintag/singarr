use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::{job::JobContext, lyrics::LyricsFilters};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveTrackParams {
    pub track_id: i64,
}

pub async fn remove_track(context: JobContext<RemoveTrackParams>) -> Result<()> {
    let track = context
        .state
        .track_service
        .find(context.params.track_id)
        .await?;

    if track.has_lyrics {
        let lyrics = context
            .state
            .lyrics_service
            .find_many(
                Some(&LyricsFilters {
                    track_id: Some(track.id),
                    album_id: None,
                    artist_id: None,
                }),
                None,
            )
            .await?;
        for lyric in lyrics {
            context.state.lyrics_service.remove(lyric.id).await?;
        }
    }

    context.state.track_service.remove(track.id).await?;

    Ok(())
}
