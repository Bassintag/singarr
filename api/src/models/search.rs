use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct CreateSearchQuery {
    pub q: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchResultKind {
    Artist,
    Album,
    Track,
}

impl TryFrom<&str> for SearchResultKind {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "artist" => SearchResultKind::Artist,
            "album" => SearchResultKind::Album,
            "track" => SearchResultKind::Track,
            _ => return Err(anyhow!("unknown search kind")),
        })
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchArtist {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAlbum {
    pub id: i64,
    pub title: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTrack {
    pub id: i64,
    pub title: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub kind: SearchResultKind,
    pub image_path: Option<String>,
    pub id: i64,
    pub artist: Option<SearchArtist>,
    pub album: Option<SearchAlbum>,
    pub track: Option<SearchTrack>,
}
