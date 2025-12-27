use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{models::job::JobContext, utils::audiodb::AudiodbClient};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncArtistMetadataParams {
    pub artist_id: i64,
    #[serde(default)]
    pub force: bool,
}

pub async fn sync_artist_metadata(context: JobContext<SyncArtistMetadataParams>) -> Result<()> {
    let with_stats = context
        .state
        .artist_service
        .find(context.params.artist_id)
        .await?;

    let artist = with_stats.artist;
    let Some(musicbrainz_id) = artist.musicbrainz_id else {
        return Ok(());
    };

    let client = AudiodbClient::new();
    let response = client.lookup_artist(&musicbrainz_id).await?;

    let Some(audiodb_artist) = response
        .artists
        .and_then(|artists| artists.into_iter().next())
    else {
        return Ok(());
    };

    let mut image_path = artist.image_path;
    if image_path.is_none() || context.params.force {
        let Some(thumb_url) = audiodb_artist.str_artist_thumb else {
            return Ok(());
        };

        let output_path = PathBuf::from("artists").join(format!("{}.webp", musicbrainz_id));

        if context
            .state
            .image_service
            .download(&thumb_url, &output_path)
            .await
            .is_ok()
        {
            image_path = Some(output_path.to_string_lossy().to_string());
        } else {
            context.log("Failed to download image");
        }
    }

    context
        .state
        .artist_service
        .set_metadata(artist.id, &image_path, &audiodb_artist.str_biography_en)
        .await?;

    Ok(())
}
