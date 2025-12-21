use std::path::PathBuf;

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

use crate::{
    models::{job::JobContext, lyrics::CreateLyrics},
    utils::checksum::md5sum,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportLyricsParams {
    pub provider: Option<String>,
    pub track_id: i64,
    pub content: String,
    pub synced: bool,
}

pub async fn import_lyrics(context: JobContext<ImportLyricsParams>) -> Result<()> {
    let settings = context.state.settings_service.get().await;
    let track = context
        .state
        .track_service
        .find(context.params.track_id)
        .await?;

    let relative_track_path = track.relative_file_path();
    let root_folder_path = PathBuf::from(&settings.root_folder);
    let track_path = root_folder_path.join(&relative_track_path);

    let Some(track_stem) = track_path.file_stem() else {
        anyhow::bail!("Invalid track path");
    };

    let Some(track_folder_path) = track_path.parent() else {
        anyhow::bail!("Track path has no parent");
    };

    let mut i = 0;

    loop {
        let output_name = if i == 0 {
            format!("{}.lrc", track_stem.to_string_lossy())
        } else {
            format!("{}.{}.lrc", track_stem.to_string_lossy(), i)
        };
        let output_path = track_folder_path.join(output_name);
        if output_path.exists() {
            i += 1;
            continue;
        }
        tokio::fs::write(&output_path, &context.params.content).await?;
        let relative_output_path = output_path.strip_prefix(settings.root_folder)?;
        context
            .state
            .lyrics_service
            .create(&CreateLyrics {
                checksum: md5sum(&context.params.content),
                file_path: relative_output_path.to_string_lossy().to_string(),
                synced: context.params.synced,
                track_id: context.params.track_id,
                provider: context.params.provider.clone(),
                language: None,
            })
            .await?;
        break;
    }

    Ok(())
}
