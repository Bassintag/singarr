use anyhow::Result;

use crate::{
    models::job::JobContext,
    worker::jobs::scan_artist::{scan_artist, ScanArtistParams},
};

pub async fn search_library(context: JobContext<()>) -> Result<()> {
    let albums = context.state.artist_service.find_many(None).await?;

    for (i, item) in albums.iter().enumerate() {
        context.log(format!(
            "[{}/{}] Scanning album: {}",
            i + 1,
            albums.len(),
            item.artist.name
        ));
        scan_artist(context.clone_with_params(ScanArtistParams {
            artist_id: item.artist.id,
        }))
        .await?;
    }

    Ok(())
}
