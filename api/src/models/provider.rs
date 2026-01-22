use anyhow::Result;
use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    models::track::Track,
    worker::{provider::LyricsProvider, providers::lrclib::LrcLibProvider},
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderMetadata {
    pub name: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderFile {
    pub identifier: String,
    pub name: String,
    pub track_name: String,
    pub artist_name: String,
    pub album_title: String,
    pub synced: bool,
    pub duration_ms: Option<i64>,
    pub content: Option<String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResult {
    pub provider: ProviderMetadata,
    pub file: ProviderFile,
    pub score: f64,
}

pub enum Provider {
    LrcLib(LrcLibProvider),
}

#[async_trait]
impl LyricsProvider for Provider {
    fn name(&self) -> &'static str {
        match self {
            Self::LrcLib(provider) => provider.name(),
        }
    }

    async fn search_lyrics(&self, track: &Track) -> Result<Vec<ProviderFile>> {
        match self {
            Self::LrcLib(provider) => provider.search_lyrics(track),
        }
        .await
    }

    async fn download(&self, result: &ProviderFile) -> Result<String> {
        match self {
            Self::LrcLib(provider) => provider.download(result),
        }
        .await
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsQuery {
    pub track_id: i64,
}
