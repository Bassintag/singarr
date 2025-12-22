use anyhow::Result;
use sqlx::{FromRow, Pool, QueryBuilder, Sqlite};

use crate::models::{
    album::{Album, AlbumWithStats, AlbumsFilters, AlbumsQuery},
    artist::Artist,
    generic::{IdRow, Page, TotalRow, TrackStats},
    lidarr::LidarrAlbum,
};

#[derive(FromRow)]
pub struct AlbumRow {
    pub id: i64,
    pub title: String,
    pub cover_path: Option<String>,
    pub lidarr_id: Option<i64>,
    pub musicbrainz_id: Option<String>,

    pub artist_id: i64,
    pub artist_name: String,
    pub artist_lidarr_id: Option<i64>,
    pub artist_musicbrainz_id: Option<String>,

    pub tracks_count: Option<i64>,
    pub with_lyrics_count: Option<i64>,
}

impl From<AlbumRow> for AlbumWithStats {
    fn from(value: AlbumRow) -> Self {
        Self {
            album: Album {
                id: value.id,
                title: value.title,
                cover_path: value.cover_path,
                lidarr_id: value.lidarr_id,
                musicbrainz_id: value.musicbrainz_id,
            },
            artist: Artist {
                id: value.artist_id,
                name: value.artist_name,
                lidarr_id: value.artist_lidarr_id,
                musicbrainz_id: value.artist_musicbrainz_id,
            },
            stats: TrackStats {
                tracks_count: value.tracks_count.unwrap_or(0),
                with_lyrics_count: value.with_lyrics_count.unwrap_or(0),
            },
        }
    }
}

const SELECT: &str = r#"SELECT
    al."id",
    al."title",
    al."cover_path",
    al."lidarr_id",
    al."musicbrainz_id",

    ar."id" as "artist_id",
    ar."name" as "artist_name",
    ar."lidarr_id" as "artist_lidarr_id",
    ar."musicbrainz_id" as "artist_musicbrainz_id",

    COUNT(DISTINCT t."id") AS "tracks_count",
    COUNT(DISTINCT l."track_id") AS "with_lyrics_count"
FROM album al
INNER JOIN artist ar ON al."artist_id" = ar."id"
LEFT JOIN track t ON t."album_id" = al."id"
LEFT JOIN lyrics l ON l."track_id" = t."id""#;

#[derive(Clone)]
pub struct AlbumSerivce {
    pool: Pool<Sqlite>,
}

impl AlbumSerivce {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    fn push_filters(qb: &mut QueryBuilder<'_, Sqlite>, filters: &AlbumsFilters) {
        if let Some(artist_id) = filters.artist_id {
            qb.push(r#" WHERE al."artist_id" = "#).push_bind(artist_id);
        }
    }

    pub async fn count(&self, filters: &AlbumsFilters) -> Result<i64> {
        let mut qb = sqlx::QueryBuilder::new(
            r#"SELECT
                COUNT(*) as "total"
            FROM album al"#,
        );
        Self::push_filters(&mut qb, filters);
        let row: TotalRow = qb.build_query_as().fetch_one(&self.pool).await?;
        Ok(row.total)
    }

    pub async fn find_many(&self, query: &AlbumsQuery) -> Result<Vec<AlbumWithStats>> {
        let (limit, offset) = query.pageable.to_limit_offset();
        let mut qb = sqlx::QueryBuilder::new(SELECT);
        Self::push_filters(&mut qb, &query.filters);
        qb.push(
            r#" GROUP BY al."id" 
            ORDER BY ar."name" ASC, al."title" ASC"#,
        );
        qb.push(" LIMIT ").push_bind(limit);
        qb.push(" OFFSET ").push_bind(offset);
        let rows: Vec<AlbumRow> = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(AlbumWithStats::from).collect())
    }

    pub async fn find_page(&self, query: &AlbumsQuery) -> Result<Page<AlbumWithStats>> {
        let (total, items) = tokio::try_join!(self.count(&query.filters), self.find_many(query))?;
        Ok(Page { total, items })
    }

    pub async fn find(&self, id: i64) -> Result<AlbumWithStats> {
        let query = format!(
            r#"{SELECT} 
            WHERE al."id" = $1"#
        );
        let row: AlbumRow = sqlx::query_as(query.as_str())
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.into())
    }

    pub async fn upsert_lidarr(&self, data: &LidarrAlbum) -> Result<i64> {
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
        Ok(row.id)
    }
}
