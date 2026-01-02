use crate::utils::de::de_opt_i64;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, QueryBuilder, Sqlite};

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
    fn to_limit_offset(&self) -> (Option<i64>, Option<i64>) {
        let page = self.page.unwrap_or(0);
        (self.size, self.size.map(|s| s * page))
    }

    pub fn push_limit_offset(&self, qb: &mut QueryBuilder<'_, Sqlite>) {
        let (limit_opt, offset_opt) = self.to_limit_offset();
        if let Some(limit) = limit_opt {
            qb.push(" LIMIT ").push_bind(limit);
        }
        if let Some(offset) = offset_opt {
            qb.push(" OFFSET ").push_bind(offset);
        }
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
