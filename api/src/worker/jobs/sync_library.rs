use anyhow::Result;

use crate::{
    models::{job::JobContext, lidarr::LidarrArtist},
    worker::jobs::{
        remove_artist::{remove_artist, RemoveArtistParams},
        sync_artist::{sync_artist, SyncArtistParams},
    },
};

pub async fn sync_library(context: JobContext<()>) -> Result<()> {
    let artists = context
        .state
        .lidarr_service
        .list_artists(None)
        .await?
        .into_iter()
        .filter(|a| a.statistics.track_file_count > 0)
        .collect::<Vec<LidarrArtist>>();

    let mut ids = Vec::new();

    for (i, lidarr_artist) in artists.iter().enumerate() {
        context.log(format!(
            "[{}/{}] Syncing artist: {}",
            i + 1,
            artists.len(),
            lidarr_artist.artist_name
        ));
        let artist_id = context
            .state
            .artist_service
            .upsert_lidarr(&lidarr_artist)
            .await?;
        sync_artist(context.clone_with_params(SyncArtistParams { artist_id })).await?;
        ids.push(artist_id);
    }

    let missing_artists = context.state.artist_service.find_excluding(&ids).await?;

    if missing_artists.len() > 0 {
        // context.log("Removing missing artists");
        for missing in missing_artists {
            println!("Removing artist {}", missing.id);
            remove_artist(context.clone_with_params(RemoveArtistParams {
                artist_id: missing.id,
            }))
            .await?;
        }
    }

    Ok(())
}
