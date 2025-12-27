use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::job::JobContext,
    worker::{
        jobs::import_lyrics::{import_lyrics, ImportLyricsParams},
        provider::{Provider, SearchResult},
        providers::lrclib::LrcLibProvider,
        score::score_result,
    },
};

struct ScoredResult {
    pub score: f64,
    pub result: SearchResult,
}

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

    let provider = LrcLibProvider::new();

    let results = provider.search_lyrics(&track).await?;

    for result in &results {
        println!(
            "Found result with score {:.2}: {:} - {:} - {:} (synced: {:})",
            score_result(&track, &result),
            result.artist_name,
            result.album_title,
            result.track_name,
            result.synced,
        );
    }

    let scored = results.into_iter().map(|result| ScoredResult {
        score: score_result(&track, &result),
        result,
    });

    let mut best_opt: Option<ScoredResult> = None;
    for item in scored {
        if item.score < settings.lyrics.min_score {
            continue;
        }
        if let Some(best) = &best_opt {
            if (!best.result.synced && item.result.synced) || best.score < item.score {
                best_opt = Some(item);
            }
        } else {
            best_opt = Some(item)
        }
    }

    if let Some(best) = best_opt {
        let best_result = best.result;
        let content = provider.download(&best_result).await?;
        import_lyrics(context.clone_with_params(ImportLyricsParams {
            provider: Some(provider.name()),
            track_id: context.params.track_id,
            content,
            synced: best_result.synced,
        }))
        .await?;
    }

    Ok(())
}
