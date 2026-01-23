use crate::utils::de::de_opt_i64;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::time::OffsetDateTime};

use crate::models::{
    artist::Artist,
    generic::{Pageable, TrackStats},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub cover_path: Option<String>,
    pub lidarr_id: Option<i64>,
    pub musicbrainz_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumWithArtist {
    #[serde(flatten)]
    pub album: Album,
    pub artist: Artist,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumWithStats {
    #[serde(flatten)]
    pub album: Album,
    pub artist: Artist,
    pub stats: TrackStats,
}

#[derive(FromRow)]
pub struct AlbumForJob {
    pub id: i64,
    pub lidarr_id: Option<i64>,
    pub metadata_updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumsFilters {
    #[serde(default, deserialize_with = "de_opt_i64")]
    pub artist_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumsQuery {
    #[serde(flatten)]
    pub pageable: Pageable,
    #[serde(flatten)]
    pub filters: AlbumsFilters,
}
