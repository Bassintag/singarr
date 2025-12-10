use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Lyrics {
    pub id: i64,
    pub synced: bool,
    pub file_path: String,
    pub checksum: String,
}

pub struct CreateLyrics {
    pub language: Option<String>,
    pub provider: Option<String>,
    pub synced: bool,
    pub file_path: String,
    pub checksum: String,
    pub track_id: i64,
}
