use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::models::{album::Album, generic::IdRow, lidarr::LidarrAlbum};

#[derive(Clone)]
pub struct AlbumSerivce {
    pool: Pool<Sqlite>,
}

impl AlbumSerivce {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Album>> {
        let rows = sqlx::query_as!(
            Album,
            r#"SELECT
                "id",
                "title",
                "cover_path",
                "lidarr_id",
                "musicbrainz_id",
                "artist_id"
            FROM album
            ORDER BY "title" ASC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn find(&self, id: i64) -> Result<Album> {
        let row = sqlx::query_as!(
            Album,
            r#"SELECT
                "id",
                "title",
                "cover_path",
                "lidarr_id",
                "musicbrainz_id",
                "artist_id"
            FROM album
            WHERE "id" = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn upsert_lidarr(&self, data: &LidarrAlbum) -> Result<Album> {
        let row = sqlx::query_as!(
            IdRow,
            r#"INSERT INTO album (
                "title",
                "lidarr_id",
                "musicbrainz_id",
                "artist_id"
            ) VALUES (
                $1, $2, $3,
                (SELECT id FROM artist WHERE lidarr_id = $4)
            ) ON CONFLICT(lidarr_id) DO UPDATE SET
                "title" = $1,
                "musicbrainz_id" = $3
            RETURNING "id""#,
            data.title,
            data.id,
            data.foreign_album_id,
            data.artist_id,
        )
        .fetch_one(&self.pool)
        .await?;
        self.find(row.id).await
    }
}
