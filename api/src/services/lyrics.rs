use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::models::{
    generic::IdRow,
    lyrics::{CreateLyrics, Lyrics},
};

#[derive(Clone)]
pub struct LyricsService {
    pool: Pool<Sqlite>,
}

impl LyricsService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Lyrics>> {
        let rows = sqlx::query_as!(
            Lyrics,
            r#"SELECT 
                "id", 
                "synced", 
                "file_path", 
                "checksum" 
            FROM lyrics
            ORDER BY "created_at" ASC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn find(&self, id: i64) -> Result<Lyrics> {
        let row = sqlx::query_as!(
            Lyrics,
            r#"SELECT 
                "id", 
                "synced", 
                "file_path", 
                "checksum" 
            FROM lyrics
            WHERE "id" = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn find_by_path(&self, path: &String) -> Result<Option<Lyrics>> {
        let row = sqlx::query_as!(
            Lyrics,
            r#"SELECT 
                "id", 
                "synced", 
                "file_path", 
                "checksum" 
            FROM lyrics
            WHERE "file_path" = $1"#,
            path
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn create(&self, data: &CreateLyrics) -> Result<Lyrics> {
        let row = sqlx::query_as!(
            IdRow,
            r#"INSERT INTO lyrics (
                "language", 
                "provider", 
                "synced", 
                "file_path", 
                "checksum", 
                "track_id"
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            ) RETURNING "id""#,
            data.language,
            data.provider,
            data.synced,
            data.file_path,
            data.checksum,
            data.track_id
        )
        .fetch_one(&self.pool)
        .await?;
        self.find(row.id).await
    }
}
