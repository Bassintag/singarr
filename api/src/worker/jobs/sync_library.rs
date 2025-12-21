use anyhow::Result;

use crate::{
    models::job::JobContext,
    worker::jobs::sync_artist::{sync_artist, SyncArtistParams},
};

pub async fn sync_library(context: JobContext<()>) -> Result<()> {
    let artists = context.state.lidarr_service.list_artists(None).await?;

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
    }

    Ok(())
}
