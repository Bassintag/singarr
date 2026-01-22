use anyhow::{anyhow, Result};
use futures::{stream::FuturesUnordered, TryStreamExt};

use crate::{
    models::{
        provider::{Provider, ProviderFile, ProviderMetadata, ProviderResult},
        track::Track,
    },
    worker::{provider::LyricsProvider, providers::lrclib::LrcLibProvider},
};

fn score_strings(a: &String, b: &String) -> f64 {
    strsim::jaro_winkler(a, b)
}

fn score_durations(a: i64, b: i64) -> f64 {
    let diff = (a - b).abs();
    (1.0 - (diff as f64 / 10_000.0)).clamp(0.0, 1.0)
}

pub struct ProviderService {
    providers: Vec<Provider>,
}

impl ProviderService {
    pub fn new() -> Self {
        Self {
            providers: vec![Provider::LrcLib(LrcLibProvider::new())],
        }
    }

    pub fn score(track: &Track, file: &ProviderFile) -> f64 {
        let score_track_name = score_strings(&track.title, &file.track_name);
        let score_artist_name = score_strings(&track.artist.name, &file.artist_name);
        let score_album_title = score_strings(&track.album.album.title, &file.album_title);
        let score_duration = match file.duration_ms {
            Some(duration_ms) => score_durations(track.duration_ms, duration_ms),
            None => 0.5,
        };

        score_duration * 0.4
            + score_track_name * 0.2
            + score_artist_name * 0.2
            + score_album_title * 0.2
    }

    pub async fn get_provider_results(
        &self,
        track: &Track,
        provider: &Provider,
    ) -> Result<Vec<ProviderResult>> {
        let mut results = provider
            .search_lyrics(track)
            .await?
            .into_iter()
            .map(|file| ProviderResult {
                provider: ProviderMetadata {
                    name: provider.name().into(),
                },
                score: Self::score(&track, &file),
                file,
            })
            .collect::<Vec<ProviderResult>>();
        results.sort_by(|a, b| b.score.total_cmp(&a.score));
        Ok(results)
    }

    pub async fn get_results(&self, track: &Track) -> Result<Vec<ProviderResult>> {
        let mut futures = FuturesUnordered::new();

        for provider in &self.providers {
            futures.push(self.get_provider_results(track, &provider));
        }

        let mut results = Vec::<ProviderResult>::new();

        while let Some(files) = futures.try_next().await? {
            for result in files.into_iter() {
                results.push(result);
            }
        }

        Ok(results)
    }

    pub async fn download(&self, result: &ProviderResult) -> Result<String> {
        if let Some(provider) = self
            .providers
            .iter()
            .find(|provider| provider.name() == result.provider.name.as_str())
        {
            Ok(provider.download(&result.file).await?)
        } else {
            Err(anyhow!("Provider not found"))
        }
    }
}
