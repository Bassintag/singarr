use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover_path: Option<String>,
    pub lidarr_id: Option<i64>,
    pub musicbrainz_id: Option<String>,
    pub artist_id: i64,
}
