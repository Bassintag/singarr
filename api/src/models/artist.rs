use serde::Serialize;
use sqlx::{types::time::OffsetDateTime, FromRow};

use crate::models::generic::TrackStats;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub lidarr_id: Option<i64>,
    pub musicbrainz_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ArtistWithStats {
    #[serde(flatten)]
    pub artist: Artist,
    pub stats: TrackStats,
}

#[derive(FromRow)]
pub struct ArtistForJob {
    pub id: i64,
    pub lidarr_id: Option<i64>,
    pub metadata_updated_at: Option<OffsetDateTime>,
}
