use crate::{models::generic::Pageable, utils::de::de_opt_i64};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    pub id: i64,
    pub synced: bool,
    pub file_path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsContent {
    pub text: String,
}

pub struct CreateLyrics {
    pub language: Option<String>,
    pub provider: Option<String>,
    pub synced: bool,
    pub file_path: String,
    pub checksum: String,
    pub track_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsFilters {
    #[serde(default, deserialize_with = "de_opt_i64")]
    pub artist_id: Option<i64>,
    #[serde(default, deserialize_with = "de_opt_i64")]
    pub album_id: Option<i64>,
    #[serde(default, deserialize_with = "de_opt_i64")]
    pub track_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsQuery {
    #[serde(flatten)]
    pub pageable: Pageable,
    #[serde(flatten)]
    pub filters: LyricsFilters,
}
