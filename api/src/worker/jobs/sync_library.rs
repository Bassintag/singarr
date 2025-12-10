use anyhow::Result;

use crate::{
    models::lidarr::{LidarrAlbumQuery, LidarrTrackFileQuery, LidarrTrackQuery},
    state::AppState,
    worker::{job::Job, jobs::sync_track::SyncTrackParams},
};

pub async fn sync_library(state: &AppState) -> Result<()> {
    let artists = state.lidarr_service.list_artists(None).await?;

    for lidarr_artist in artists.iter() {
        if lidarr_artist.statistics.track_file_count == 0 {
            continue;
        }
        println!("Found artist: {}", lidarr_artist.artist_name);
        state.artist_service.upsert_lidarr(&lidarr_artist).await?;

        let track_file_query = LidarrTrackFileQuery {
            artist_id: Some(lidarr_artist.id),
            ..Default::default()
        };

        let lidarr_track_files = state
            .lidarr_service
            .list_track_files(Some(&track_file_query))
            .await?;

        if lidarr_track_files.len() == 0 {
            continue;
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

        let lidarr_albums = state.lidarr_service.list_albums(Some(&album_query)).await?;

        for lidarr_album in lidarr_albums.iter() {
            println!("Found album: {}", lidarr_album.title);
            state.album_service.upsert_lidarr(&lidarr_album).await?;
        }

        let track_query = LidarrTrackQuery {
            artist_id: Some(lidarr_artist.id),
            ..Default::default()
        };

        let lidarr_tracks = state.lidarr_service.list_tracks(Some(&track_query)).await?;

        for lidarr_track_file in lidarr_track_files.iter() {
            if let Some(lidarr_track) = lidarr_tracks
                .iter()
                .find(|t| t.track_file_id == lidarr_track_file.id)
            {
                println!("Found track: {}", lidarr_track.title);
                let track = state
                    .track_service
                    .upsert_lidarr(&lidarr_track, &lidarr_track_file)
                    .await?;
                state
                    .queue
                    .enqueue(Job::SyncTrack(SyncTrackParams { track_id: track.id }))?;
            }
        }
    }

    Ok(())
}
