use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        job::JobContext,
        lidarr::{LidarrAlbum, LidarrAlbumQuery},
    },
    worker::jobs::{
        remove_album::{remove_album, RemoveAlbumParams},
        sync_album::{sync_album, SyncAlbumParams},
        sync_artist_metadata::{sync_artist_metadata, SyncArtistMetadataParams},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncArtistParams {
    pub artist_id: i64,
}

pub async fn sync_artist(context: JobContext<SyncArtistParams>) -> Result<()> {
    let artist = context
        .state
        .artist_service
        .find(context.params.artist_id)
        .await?;

    if artist.artist.lidarr_id.is_none() {
        return Ok(());
    }

    let album_query = LidarrAlbumQuery {
        artist_id: artist.artist.lidarr_id,
        ..Default::default()
    };

    let albums = context
        .state
        .lidarr_service
        .list_albums(Some(&album_query))
        .await?
        .into_iter()
        .filter(|a| {
            a.statistics
                .as_ref()
                .map(|s| s.track_file_count > 0)
                .unwrap_or(false)
        })
        .collect::<Vec<LidarrAlbum>>();

    let mut ids = Vec::new();

    for (i, lidarr_album) in albums.iter().enumerate() {
        context.log(format!(
            "[{}/{}] Syncing album: {}",
            i + 1,
            albums.len(),
            lidarr_album.title
        ));
        let album_id = context
            .state
            .album_service
            .upsert_lidarr(&lidarr_album)
            .await?;
        sync_album(context.clone_with_params(SyncAlbumParams { album_id })).await?;
        ids.push(album_id);
    }

    let missing_albums = context
        .state
        .album_service
        .find_excluding(context.params.artist_id, &ids)
        .await?;

    if missing_albums.len() > 0 {
        // context.log("Removing missing albums");
        for missing in missing_albums {
            println!("Removing album {}", missing.id);
            remove_album(context.clone_with_params(RemoveAlbumParams {
                album_id: missing.id,
            }))
            .await?;
        }
    }

    sync_artist_metadata(context.clone_with_params(SyncArtistMetadataParams {
        artist_id: context.params.artist_id,
        force: false,
    }))
    .await?;

    Ok(())
}
