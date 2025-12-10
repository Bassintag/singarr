use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::models::{artist::Artist, generic::IdRow, lidarr::LidarrArtist};

#[derive(Clone)]
pub struct ArtistSerivce {
    pool: Pool<Sqlite>,
}

impl ArtistSerivce {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Artist>> {
        let rows = sqlx::query_as!(
            Artist,
            r#"SELECT
                "id",
                "name",
                "lidarr_id",
                "musicbrainz_id"
            FROM artist
            ORDER BY "name" ASC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn find(&self, id: i64) -> Result<Artist> {
        let row = sqlx::query_as!(
            Artist,
            r#"SELECT
                "id",
                "name",
                "lidarr_id",
                "musicbrainz_id"
            FROM artist
            WHERE id = $1"#,
            id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn upsert_lidarr(&self, data: &LidarrArtist) -> Result<Artist> {
        let row = sqlx::query_as!(
            IdRow,
            r#"INSERT INTO artist (
                "name",
                "lidarr_id",
                "musicbrainz_id"
            ) VALUES (
             $1, $2, $3
            ) ON CONFLICT(lidarr_id) DO UPDATE SET
                "name" = $1,
                "musicbrainz_id" = $3
            RETURNING "id""#,
            data.artist_name,
            data.id,
            data.foreign_artist_id
        )
        .fetch_one(&self.pool)
        .await?;
        self.find(row.id).await
    }
}
