use anyhow::Result;
use async_trait::async_trait;

use crate::models::{provider::ProviderFile, track::Track};

#[async_trait]
pub trait LyricsProvider: Send + Sync {
    fn name(&self) -> &'static str;
    async fn search_lyrics(&self, track: &Track) -> Result<Vec<ProviderFile>>;
    async fn download(&self, result: &ProviderFile) -> Result<String>;
}
