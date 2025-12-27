use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        job::JobContext,
        lidarr::{LidarrAlbumQuery, LidarrTrackFileQuery, LidarrTrackQuery},
    },
    worker::jobs::{
        sync_album_metadata::{sync_album_metadata, SyncAlbumMetadataParams},
        sync_artist_metadata::{sync_artist_metadata, SyncArtistMetadataParams},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncArtistParams {
    pub artist_id: i64,
}

pub async fn sync_artist(context: JobContext<SyncArtistParams>) -> Result<()> {
    let with_stats = context
        .state
        .artist_service
        .find(context.params.artist_id)
        .await?;

    let artist = with_stats.artist;

    if artist.image_path.is_none() || artist.description.is_none() {
        sync_artist_metadata(context.clone_with_params(SyncArtistMetadataParams {
            artist_id: artist.id,
            force: false,
        }))
        .await?;
    }

    let Some(artist_lidarr_id) = artist.lidarr_id else {
        return Ok(());
    };

    let track_file_query = LidarrTrackFileQuery {
        artist_id: Some(artist_lidarr_id),
        ..Default::default()
    };

    let lidarr_track_files = context
        .state
        .lidarr_service
        .list_track_files(Some(&track_file_query))
        .await?;

    if lidarr_track_files.len() == 0 {
        return Ok(());
    }

    let mut album_ids = lidarr_track_files
        .iter()
        .map(|f| f.album_id)
        .collect::<Vec<i64>>();
    album_ids.sort();
    album_ids.dedup();

    let album_query = LidarrAlbumQuery {
        album_ids: Some(album_ids),
        ..Default::default()
    };

    let lidarr_albums = context
        .state
        .lidarr_service
        .list_albums(Some(&album_query))
        .await?;

    for lidarr_album in lidarr_albums.iter() {
        println!("Found album: {}", lidarr_album.title);
        let album_id = context
            .state
            .album_service
            .upsert_lidarr(&lidarr_album)
            .await?;
        let with_stats = context.state.album_service.find(album_id).await?;
        if with_stats.album.cover_path.is_none() {
            sync_album_metadata(context.clone_with_params(SyncAlbumMetadataParams {
                album_id: with_stats.album.id,
            }))
            .await?
        }
    }

    let track_query = LidarrTrackQuery {
        artist_id: Some(artist_lidarr_id),
        ..Default::default()
    };

    let lidarr_tracks = context
        .state
        .lidarr_service
        .list_tracks(Some(&track_query))
        .await?;

    for lidarr_track_file in lidarr_track_files.iter() {
        if let Some(lidarr_track) = lidarr_tracks
            .iter()
            .find(|t| t.track_file_id == lidarr_track_file.id)
        {
            println!("Found track: {}", lidarr_track.title);
            context
                .state
                .track_service
                .upsert_lidarr(&lidarr_track, &lidarr_track_file)
                .await?;
        }
    }

    Ok(())
}
