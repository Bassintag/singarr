use std::path::PathBuf;

use anyhow::{Ok, Result};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

use crate::{models::lyrics::CreateLyrics, state::AppState};

#[derive(Debug)]
pub enum ImportType {
    File(PathBuf),
    Memory(String),
}

#[derive(Debug)]
pub struct ImportLyricsParams {
    pub provider: Option<String>,
    pub track_id: i64,
    pub import_type: ImportType,
}

pub async fn import_lyrics(state: &AppState, params: &ImportLyricsParams) -> Result<()> {
    let settings = state.settings_service.get().await;
    let track = state.track_service.find(params.track_id).await?;

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
        match &params.import_type {
            ImportType::File(input_path) => {
                if &output_path != input_path {
                    println!("{:?} <> {:?}", &output_path, input_path);
                    tokio::fs::copy(&input_path, &output_path).await?;
                }
            }
            ImportType::Memory(content) => {
                tokio::fs::write(&output_path, content).await?;
            }
        };
        let output_path_relative = output_path.strip_prefix(&root_folder_path)?;
        state
            .lyrics_service
            .create(&CreateLyrics {
                language: None,
                provider: params.provider.clone(),
                synced: false,
                file_path: output_path_relative.to_string_lossy().into(),
                checksum: md5sum(&output_path).await?,
                track_id: params.track_id,
            })
            .await?;
        break;
    }

    Ok(())
}

async fn md5sum(path: &PathBuf) -> Result<String> {
    let file = File::open(path).await?;
    let mut reader = BufReader::new(file);

    let mut context = md5::Context::new();
    let mut buffer = [0u8; 8192];

    loop {
        let read = reader.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        context.consume(&buffer[..read]);
    }

    let digest = context.finalize();
    Ok(format!("{:x}", digest))
}
