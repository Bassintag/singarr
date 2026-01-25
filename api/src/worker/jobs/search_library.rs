use anyhow::Result;

use crate::{
    models::job::JobContext,
    worker::jobs::search_artist::{search_artist, SearchArtistParams},
};

pub async fn search_library(context: JobContext<()>) -> Result<()> {
    let albums = context.state.artist_service.find_many(None).await?;

    for (i, item) in albums.iter().enumerate() {
        context.log(format!(
            "[{}/{}] Searching album: {}",
            i + 1,
            albums.len(),
            item.artist.name
        ));
        if let Err(e) = search_artist(context.clone_with_params(SearchArtistParams {
            artist_id: item.artist.id,
        }))
        .await
        {
            eprintln!("Error while searching album {}: {}", item.artist.name, e);
        }
    }

    Ok(())
}
