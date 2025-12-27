use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{album::AlbumsFilters, job::JobContext},
    worker::jobs::search_album::{search_album, SearchAlbumParams},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchArtistParams {
    pub artist_id: i64,
}

pub async fn search_artist(context: JobContext<SearchArtistParams>) -> Result<()> {
    let albums = context
        .state
        .album_service
        .find_many(
            Some(&AlbumsFilters {
                artist_id: Some(context.params.artist_id),
            }),
            None,
        )
        .await?;

    for album in albums.iter() {
        if album.stats.with_lyrics_count < album.stats.tracks_count {
            search_album(context.clone_with_params(SearchAlbumParams {
                album_id: album.album.id,
            }))
            .await?;
        }
    }

    Ok(())
}
