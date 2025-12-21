use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        album::{AlbumsFilters, AlbumsQuery},
        job::JobContext,
    },
    worker::jobs::scan_album::{scan_album, ScanAlbumParams},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanArtistParams {
    pub artist_id: i64,
}

pub async fn scan_artist(context: JobContext<ScanArtistParams>) -> Result<()> {
    let albums = context
        .state
        .album_service
        .find_many(&AlbumsQuery {
            filters: AlbumsFilters {
                artist_id: Some(context.params.artist_id),
            },
            pageable: Default::default(),
        })
        .await?;

    for (i, album) in albums.iter().enumerate() {
        context.log(format!(
            "[{}/{}] Scanning album: {}",
            i + 1,
            albums.len(),
            album.album.title
        ));
        scan_album(context.clone_with_params(ScanAlbumParams {
            album_id: album.album.id,
        }))
        .await?;
    }

    Ok(())
}
