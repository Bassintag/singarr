use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub lidarr_id: Option<i64>,
    pub musicbrainz_id: Option<String>,
}
