use anyhow::Result;
use async_trait::async_trait;

use crate::models::track::Track;

pub struct SearchResult {
    pub identifier: String,
    pub track_name: String,
    pub artist_name: String,
    pub album_title: String,
    pub synced: bool,
    pub duration_ms: Option<i64>,
}

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> String;
    async fn search_lyrics(&self, track: &Track) -> Result<Vec<SearchResult>>;
    async fn download(&self, result: &SearchResult) -> Result<String>;
}
