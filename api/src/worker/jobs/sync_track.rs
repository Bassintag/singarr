use std::path::PathBuf;

use anyhow::Result;

use crate::{
    state::AppState,
    worker::{
        job::Job,
        jobs::import_lyrics::{ImportLyricsParams, ImportType},
    },
};

#[derive(Debug)]
pub struct SyncTrackParams {
    pub track_id: i64,
}

pub async fn sync_track(state: &AppState, params: &SyncTrackParams) -> Result<()> {
    let settings = state.settings_service.get().await;
    let track = state.track_service.find(params.track_id).await?;

    let relative_track_path = track.relative_file_path();
    let root_folder_path = PathBuf::from(&settings.root_folder);
    let track_path = root_folder_path.join(&relative_track_path);

    let Some(track_stem) = track_path.file_stem() else {
        anyhow::bail!("Invalid track path");
    };
    let track_stem_str = track_stem.to_string_lossy();

    let Some(track_folder_path) = track_path.parent() else {
        anyhow::bail!("Track path has no parent");
    };

    println!("LISTING {:?}", track_folder_path);

    let mut read_dir = tokio::fs::read_dir(track_folder_path).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        let file_path = entry.path();
        let Some(file_ext) = file_path.extension() else {
            continue;
        };
        if file_ext != "lrc" {
            continue;
        }
        let Some(file_stem) = file_path.file_stem() else {
            continue;
        };
        let file_stem_str = file_stem.to_string_lossy();
        if !file_stem_str.starts_with(&*track_stem_str) {
            continue;
        }

        let relative_path = file_path.strip_prefix(&root_folder_path)?;
        let relative_path_string = String::from(relative_path.to_string_lossy());

        if state
            .lyrics_service
            .find_by_path(&relative_path_string)
            .await?
            .is_none()
        {
            state.queue.enqueue(Job::ImportLyrics(ImportLyricsParams {
                provider: None,
                track_id: params.track_id,
                import_type: ImportType::File(file_path),
            }))?;
        }
    }

    Ok(())
}
