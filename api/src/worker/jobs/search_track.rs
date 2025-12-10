use anyhow::Result;

use crate::{
    state::AppState,
    worker::{
        jobs::import_lyrics::{ImportLyricsParams, ImportType},
        provider::{Provider, SearchResult},
        providers::lrclib::LrcLibProvider,
        score::score_result,
    },
};

struct ScoredResult {
    pub score: f64,
    pub result: SearchResult,
}

#[derive(Debug)]
pub struct SearchTrackParams {
    pub track_id: i64,
}

pub async fn search_track(state: &AppState, params: &SearchTrackParams) -> Result<()> {
    let track = state.track_service.find(params.track_id).await?;

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

    let best = results
        .into_iter()
        .map(|result| ScoredResult {
            score: score_result(&track, &result),
            result,
        })
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

    if let Some(best_scored) = best {
        let best_result = best_scored.result;
        let content = provider.download(&best_result).await?;
        state
            .queue
            .enqueue(crate::worker::job::Job::ImportLyrics(ImportLyricsParams {
                provider: Some(provider.name()),
                track_id: params.track_id,
                import_type: ImportType::Memory(content),
            }))?;
    }

    Ok(())
}
