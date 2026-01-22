use crate::{
    models::{provider::ProviderFile, track::Track},
    worker::provider::LyricsProvider,
};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Default)]
pub struct LrcLibLyricsQuery {
    pub q: Option<String>,
    pub track_name: Option<String>,
    pub artist_name: Option<String>,
    pub album_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LrcLibLyrics {
    pub id: i64,
    pub name: String,
    pub track_name: String,
    pub artist_name: String,
    pub album_name: String,
    pub duration: f64,
    pub instrumental: bool,
    pub plain_lyrics: String,
    pub synced_lyrics: Option<String>,
}

pub struct LrcLibProvider {
    base_url: String,
    client: Client,
}

impl LrcLibProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://lrclib.net/".into(),
            client: Client::new(),
        }
    }

    pub async fn request<T: DeserializeOwned, Q: Serialize>(
        &self,
        path: &str,
        query: Option<Q>,
    ) -> Result<T> {
        let mut url = Url::parse(&self.base_url).unwrap();
        url.set_path(path);
        if let Some(q) = query {
            let query_string = serde_qs::to_string(&q)?;
            url.set_query(Some(&query_string));
        }
        println!("[LRCLIB] Fetch '{}'", url);
        let builder = self.client.get(url);
        Ok(builder.send().await?.json::<T>().await?)
    }

    pub async fn search(&self, query: Option<LrcLibLyricsQuery>) -> Result<Vec<LrcLibLyrics>> {
        self.request("api/search", query).await
    }

    pub async fn retrieve(&self, id: i64) -> Result<LrcLibLyrics> {
        self.request(&format!("api/get/{:}", id).as_str(), None::<()>)
            .await
    }
}

#[async_trait]
impl LyricsProvider for LrcLibProvider {
    fn name(&self) -> &'static str {
        "LrcLib"
    }

    async fn search_lyrics(&self, track: &Track) -> Result<Vec<ProviderFile>> {
        let results = self
            .search(Some(LrcLibLyricsQuery {
                track_name: Some(track.title.clone()),
                artist_name: Some(track.artist.name.clone()),
                album_name: Some(track.album.album.title.clone()),
                ..Default::default()
            }))
            .await?;
        Ok(results
            .into_iter()
            .map(|r| ProviderFile {
                identifier: r.id.to_string(),
                name: r.name,
                track_name: r.track_name,
                album_title: r.album_name,
                artist_name: r.artist_name,
                synced: r.synced_lyrics.is_some(),
                duration_ms: Some((r.duration * 1_000.0) as i64),
                content: Some(r.synced_lyrics.unwrap_or(r.plain_lyrics)),
            })
            .collect())
    }

    async fn download(&self, file: &ProviderFile) -> Result<String> {
        let result = self.retrieve(file.identifier.parse()?).await?;
        Ok(result.synced_lyrics.unwrap_or(result.plain_lyrics))
    }
}
