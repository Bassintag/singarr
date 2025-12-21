use anyhow::Result;
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::models::{
    artist::{Artist, ArtistWithStats},
    generic::{IdRow, Page, Pageable, TotalRow, TrackStats},
    lidarr::LidarrArtist,
};

#[derive(FromRow)]
struct ArtistRow {
    pub id: i64,
    pub name: String,
    pub lidarr_id: Option<i64>,
    pub musicbrainz_id: Option<String>,

    pub tracks_count: Option<i64>,
    pub with_lyrics_count: Option<i64>,
}

impl From<ArtistRow> for ArtistWithStats {
    fn from(value: ArtistRow) -> Self {
        Self {
            artist: Artist {
                id: value.id,
                name: value.name,
                lidarr_id: value.lidarr_id,
                musicbrainz_id: value.musicbrainz_id,
            },
            stats: TrackStats {
                tracks_count: value.tracks_count.unwrap_or(0),
                with_lyrics_count: value.with_lyrics_count.unwrap_or(0),
            },
        }
    }
}

const SELECT: &str = r#"SELECT
    ar."id",
    ar."name",
    ar."lidarr_id",
    ar."musicbrainz_id",

    COUNT(DISTINCT t."id") AS "tracks_count",
    COUNT(DISTINCT l."track_id") AS "with_lyrics_count"
FROM artist ar
LEFT JOIN track t ON t."artist_id" = ar."id"
LEFT JOIN lyrics l ON l."track_id" = t."id""#;

#[derive(Clone)]
pub struct ArtistSerivce {
    pool: Pool<Sqlite>,
}

impl ArtistSerivce {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn count(&self) -> Result<i64> {
        let row = sqlx::query_as!(TotalRow, r#"SELECT COUNT(*) as "total" FROM artist"#)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.total)
    }

    pub async fn find_many(&self, pageable: &Pageable) -> Result<Vec<ArtistWithStats>> {
        let (limit, offset) = pageable.to_limit_offset();
        let query = format!(
            r#"{SELECT}
            GROUP BY ar."id" 
            ORDER BY ar."name" ASC 
            LIMIT $1 
            OFFSET $2"#
        );
        let rows: Vec<ArtistRow> = sqlx::query_as(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(ArtistWithStats::from).collect())
    }

    pub async fn find_page(&self, pageable: &Pageable) -> Result<Page<ArtistWithStats>> {
        let (total, items) = tokio::try_join!(self.count(), self.find_many(pageable))?;
        Ok(Page { total, items })
    }

    pub async fn find(&self, id: i64) -> Result<ArtistWithStats> {
        let query = format!(
            r#"{SELECT}
            WHERE ar."id" = $1"#
        );
        let row: ArtistRow = sqlx::query_as(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.into())
    }

    pub async fn upsert_lidarr(&self, data: &LidarrArtist) -> Result<i64> {
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
        Ok(row.id)
    }
}
