use crate::utils::de::de_opt_i64;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct IdRow {
    pub id: i64,
}

#[derive(FromRow)]
pub struct TotalRow {
    pub total: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Pageable {
    #[serde(default, deserialize_with = "de_opt_i64")]
    pub page: Option<i64>,
    #[serde(default, deserialize_with = "de_opt_i64")]
    pub size: Option<i64>,
}

impl Default for Pageable {
    fn default() -> Self {
        Self {
            page: None,
            size: None,
        }
    }
}

impl Pageable {
    pub fn to_limit_offset(&self) -> (i64, i64) {
        let page = self.page.unwrap_or(0);
        let size = self.size.unwrap_or(24);
        (size, page * size)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T: Serialize> {
    pub total: i64,
    pub items: Vec<T>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackStats {
    pub tracks_count: i64,
    pub with_lyrics_count: i64,
}
