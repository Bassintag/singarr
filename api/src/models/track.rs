use crate::{models::generic::Pageable, utils::de::de_opt_i64};
use serde::{Deserialize, Serialize};

use crate::models::{album::AlbumWithArtist, artist::Artist};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i64,
    pub track_number: i64,
    pub title: String,
    pub file_path: String,
    pub duration_ms: i64,
    pub has_lyrics: bool,
    pub album: AlbumWithArtist,
    pub artist: Artist,
}

impl Track {
    pub fn relative_file_path(&self) -> String {
        if self.file_path.starts_with('/') {
            self.file_path[1..].into()
        } else {
            self.file_path.clone()
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TracksFilters {
    #[serde(default, deserialize_with = "de_opt_i64")]
    pub artist_id: Option<i64>,
    #[serde(default, deserialize_with = "de_opt_i64")]
    pub album_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TracksQuery {
    #[serde(flatten)]
    pub pageable: Pageable,
    #[serde(flatten)]
    pub filters: TracksFilters,
}
