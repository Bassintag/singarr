use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Clone, Serialize, FromRow)]
pub struct CountsStats {
    pub artist: i64,
    pub album: i64,
    pub track: i64,
}
