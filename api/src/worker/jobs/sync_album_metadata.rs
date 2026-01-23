use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{models::job::JobContext, utils::audiodb::AudiodbClient};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncAlbumMetadataParams {
    pub album_id: i64,
    #[serde(default)]
    pub force: bool,
}

pub async fn sync_album_metadata(context: JobContext<SyncAlbumMetadataParams>) -> Result<()> {
    let with_stats = context
        .state
        .album_service
        .find(context.params.album_id)
        .await?;

    let album = with_stats.album;
    let Some(musicbrainz_id) = album.musicbrainz_id else {
        return Ok(());
    };

    let client = AudiodbClient::new();
    let response = client.lookup_album(&musicbrainz_id).await?;

    let Some(audiodb_album) = response.album.and_then(|albums| albums.into_iter().next()) else {
        return Ok(());
    };

    let Some(thumb_url) = audiodb_album.str_album_thumb else {
        return Ok(());
    };

    let mut cover_path = album.cover_path;
    if cover_path.is_none() || context.params.force {
        let output_path = PathBuf::from("albums").join(format!("{}.webp", musicbrainz_id));

        if let Err(e) = context
            .state
            .image_service
            .download(&thumb_url, &output_path)
            .await
        {
            eprintln!("Error while downloading image at {}: {:}", thumb_url, e);
            context.log("Failed to download image");
            return Ok(());
        } else {
            cover_path = Some(output_path.to_string_lossy().to_string());
        }
    }

    context
        .state
        .album_service
        .set_metadata(album.id, &cover_path, &audiodb_album.str_description_en)
        .await?;

    Ok(())
}
