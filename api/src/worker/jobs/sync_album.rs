use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        job::JobContext,
        lidarr::{LidarrTrackFileQuery, LidarrTrackQuery},
    },
    worker::jobs::{
        remove_track::{remove_track, RemoveTrackParams},
        sync_album_metadata::{sync_album_metadata, SyncAlbumMetadataParams},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncAlbumParams {
    pub album_id: i64,
}

pub async fn sync_album(context: JobContext<SyncAlbumParams>) -> Result<()> {
    let album = context
        .state
        .album_service
        .find_for_job(context.params.album_id)
        .await?;

    if album.lidarr_id.is_none() {
        return Ok(());
    }

    let track_query = LidarrTrackQuery {
        album_id: album.lidarr_id,
        ..Default::default()
    };

    let track_file_query = LidarrTrackFileQuery {
        album_id: album.lidarr_id,
        ..Default::default()
    };

    let (lidarr_track_files, lidarr_tracks) = tokio::try_join!(
        context
            .state
            .lidarr_service
            .list_track_files(Some(&track_file_query)),
        context.state.lidarr_service.list_tracks(Some(&track_query))
    )?;

    let mut ids = Vec::new();

    for lidarr_track_file in lidarr_track_files.iter() {
        if let Some(lidarr_track) = lidarr_tracks
            .iter()
            .find(|t| t.track_file_id == lidarr_track_file.id)
        {
            println!("Found track: {}", lidarr_track.title);
            let track_id = context
                .state
                .track_service
                .upsert_lidarr(&lidarr_track, &lidarr_track_file)
                .await?;
            ids.push(track_id);
        }
    }

    let missing_tracks = context
        .state
        .track_service
        .find_excluding(context.params.album_id, &ids)
        .await?;

    if missing_tracks.len() > 0 {
        // context.log("Removing missing trakcs");
        for missing in missing_tracks {
            println!("Removing track {}", missing.id);
            remove_track(context.clone_with_params(RemoveTrackParams {
                track_id: missing.id,
            }))
            .await?;
        }
    }

    if album.metadata_updated_at.is_none() {
        sync_album_metadata(context.clone_with_params(SyncAlbumMetadataParams {
            album_id: context.params.album_id,
            force: false,
        }))
        .await?;
    }

    Ok(())
}
