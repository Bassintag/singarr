use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::{
    job::JobContext,
    lyrics::{Lyrics, LyricsFilters},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanAlbumParams {
    pub album_id: i64,
}

pub async fn clean_album(context: JobContext<CleanAlbumParams>) -> Result<()> {
    let settings = context.state.settings_service.get().await;
    let root_folder_path = PathBuf::from(settings.root_folder);

    let lyrics = context
        .state
        .lyrics_service
        .find_all(&LyricsFilters {
            album_id: Some(context.params.album_id),
            artist_id: None,
            track_id: None,
        })
        .await?;

    let with_files: Vec<(PathBuf, &Lyrics)> = lyrics
        .iter()
        .map(|lyrics| {
            (
                root_folder_path.join(PathBuf::from(&lyrics.file_path)),
                lyrics,
            )
        })
        .collect();

    let album_folders: HashSet<PathBuf> = lyrics
        .iter()
        .filter_map(|l| {
            PathBuf::from(&l.file_path)
                .parent()
                .map(|p| p.to_path_buf())
        })
        .collect();

    let mut keep_ids = Vec::<i64>::new();

    for album_folder in &album_folders {
        let mut read_dir = tokio::fs::read_dir(root_folder_path.join(album_folder)).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let entry_path = entry.path();
            let opt = with_files.iter().find(|(path, _)| path == &entry_path);
            if let Some((_, lyrics)) = opt {
                keep_ids.push(lyrics.id);
            }
        }
    }

    context
        .state
        .lyrics_service
        .delete_many(context.params.album_id, &keep_ids)
        .await?;

    Ok(())
}
