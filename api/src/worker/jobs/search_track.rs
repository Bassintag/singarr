use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::{job::JobContext, provider::ProviderResult},
    worker::jobs::import_lyrics::{import_lyrics, ImportLyricsParams},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTrackParams {
    pub track_id: i64,
}

pub async fn search_track(context: JobContext<SearchTrackParams>) -> Result<()> {
    let settings = context.state.settings_service.get().await;

    let track = context
        .state
        .track_service
        .find(context.params.track_id)
        .await?;

    let results = context.state.provider_service.get_results(&track).await?;

    for result in &results {
        println!(
            "Found result with score {:.2}: {:} - {:} - {:} (synced: {:})",
            result.score,
            result.file.artist_name,
            result.file.album_title,
            result.file.track_name,
            result.file.synced,
        );
    }

    let mut best_opt: Option<ProviderResult> = None;
    for result in results {
        if result.score < settings.lyrics.min_score {
            continue;
        }
        if let Some(best) = &best_opt {
            if (!best.file.synced && result.file.synced) || best.score < result.score {
                best_opt = Some(result);
            }
        } else {
            best_opt = Some(result)
        }
    }

    if let Some(best) = best_opt {
        let content = if let Some(content) = best.file.content {
            content
        } else {
            context.state.provider_service.download(&best).await?
        };
        import_lyrics(context.clone_with_params(ImportLyricsParams {
            provider: Some(best.provider.name),
            track_id: context.params.track_id,
            content,
            synced: best.file.synced,
        }))
        .await?;
    }

    Ok(())
}
