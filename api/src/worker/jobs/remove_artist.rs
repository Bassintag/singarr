use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{album::AlbumsFilters, job::JobContext},
    worker::jobs::remove_album::{remove_album, RemoveAlbumParams},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveArtistParams {
    pub artist_id: i64,
}

pub async fn remove_artist(context: JobContext<RemoveArtistParams>) -> Result<()> {
    let artist = context
        .state
        .artist_service
        .find(context.params.artist_id)
        .await?;

    let albums = context
        .state
        .album_service
        .find_many(
            Some(&AlbumsFilters {
                artist_id: Some(artist.artist.id),
            }),
            None,
        )
        .await?;

    for album in &albums {
        remove_album(context.clone_with_params(RemoveAlbumParams {
            album_id: album.album.id,
        }))
        .await?;
    }

    if let Some(image_path) = artist.artist.image_path {
        if let Err(e) = context
            .state
            .image_service
            .remove(&PathBuf::from(&image_path))
            .await
        {
            eprintln!("Failed removing image {}: {}", image_path, e)
        }
    }

    context
        .state
        .artist_service
        .remove(artist.artist.id)
        .await?;

    Ok(())
}
