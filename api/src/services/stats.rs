use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::models::stats::CountsStats;

pub struct StatsService {
    pool: Pool<Sqlite>,
}

impl StatsService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn get_counts(&self) -> Result<CountsStats> {
        Ok(sqlx::query_as!(
            CountsStats,
            r#"SELECT
                (SELECT COUNT(*) FROM artist) AS artist,
                (SELECT COUNT(*) FROM album)  AS album,
                (SELECT COUNT(*) FROM track)  AS track,
                (
                    SELECT COUNT(*)
                    FROM track t
                    LEFT JOIN lyrics l ON l."track_id" = t."id"
                    WHERE l."track_id" IS NULL
                ) AS wanted
        "#
        )
        .fetch_one(&self.pool)
        .await?)
    }
}
