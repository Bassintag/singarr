use anyhow::{Ok, Result};
use sqlx::{prelude::FromRow, Pool, QueryBuilder, Sqlite};

use crate::models::{
    album::{Album, AlbumWithArtist},
    artist::Artist,
    generic::{IdRow, Page, TotalRow},
    lidarr::{LidarrTrack, LidarrTrackFile},
    track::{Track, TracksFilters, TracksQuery},
};

#[derive(FromRow)]
struct TrackRow {
    pub id: i64,
    pub track_number: Option<i64>,
    pub title: String,
    pub file_path: String,
    pub duration_ms: Option<i64>,
    pub has_lyrics: bool,

    pub artist_id: i64,
    pub artist_name: String,
    pub artist_lidarr_id: Option<i64>,
    pub artist_musicbrainz_id: Option<String>,

    pub album_id: i64,
    pub album_title: String,
    pub album_cover_path: Option<String>,
    pub album_lidarr_id: Option<i64>,
    pub album_musicbrainz_id: Option<String>,

    pub album_artist_id: i64,
    pub album_artist_name: String,
    pub album_artist_lidarr_id: Option<i64>,
    pub album_artist_musicbrainz_id: Option<String>,
}

impl From<TrackRow> for Track {
    fn from(value: TrackRow) -> Self {
        Self {
            id: value.id,
            track_number: value.track_number.unwrap_or(0),
            title: value.title,
            file_path: value.file_path,
            duration_ms: value.duration_ms.unwrap_or(0),
            has_lyrics: value.has_lyrics,
            artist: Artist {
                id: value.artist_id,
                name: value.artist_name,
                lidarr_id: value.artist_lidarr_id,
                musicbrainz_id: value.artist_musicbrainz_id,
            },
            album: AlbumWithArtist {
                album: Album {
                    id: value.album_id,
                    title: value.album_title,
                    cover_path: value.album_cover_path,
                    lidarr_id: value.album_lidarr_id,
                    musicbrainz_id: value.album_musicbrainz_id,
                },
                artist: Artist {
                    id: value.album_artist_id,
                    name: value.album_artist_name,
                    lidarr_id: value.album_artist_lidarr_id,
                    musicbrainz_id: value.album_artist_musicbrainz_id,
                },
            },
        }
    }
}

const SELECT: &str = r#"SELECT
    t."id",
    t."track_number",
    t."title",
    t."file_path",
    t."duration_ms",
    EXISTS (SELECT 1 FROM lyrics l WHERE l."track_id" = t."id") AS "has_lyrics",

    ar."id" as "artist_id",
    ar."name" as "artist_name",
    ar."lidarr_id" as "artist_lidarr_id",
    ar."musicbrainz_id" as "artist_musicbrainz_id",

    al."id" as "album_id",
    al."title" as "album_title",
    al."cover_path" as "album_cover_path",
    al."lidarr_id" as "album_lidarr_id",
    al."musicbrainz_id" as "album_musicbrainz_id",

    al_ar."id" as "album_artist_id",
    al_ar."name" as "album_artist_name",
    al_ar."lidarr_id" as "album_artist_lidarr_id",
    al_ar."musicbrainz_id" as "album_artist_musicbrainz_id"
FROM track t
INNER JOIN artist ar ON t."artist_id" = ar."id"
INNER JOIN album al ON t."album_id" = al."id"
INNER JOIN artist al_ar ON al."artist_id" = al_ar."id""#;

#[derive(Clone)]
pub struct TrackService {
    pool: Pool<Sqlite>,
}

impl TrackService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    fn push_filters(qb: &mut QueryBuilder<'_, Sqlite>, filters: &TracksFilters) {
        qb.push(" WHERE 1=1");
        if let Some(artist_id) = filters.artist_id {
            qb.push(r#" AND t."artist_id" = "#).push_bind(artist_id);
        }
        if let Some(album_id) = filters.album_id {
            qb.push(r#" AND t."album_id" = "#).push_bind(album_id);
        }
    }

    pub async fn count(&self, filters: &TracksFilters) -> Result<i64> {
        let mut qb = sqlx::QueryBuilder::new(
            r#"SELECT
                COUNT(*) as "total"
            FROM track t"#,
        );
        Self::push_filters(&mut qb, filters);
        let row: TotalRow = qb.build_query_as().fetch_one(&self.pool).await?;
        Ok(row.total)
    }

    pub async fn find_many(&self, query: &TracksQuery) -> Result<Vec<Track>> {
        let (limit, offset) = query.pageable.to_limit_offset();
        let mut qb = sqlx::QueryBuilder::new(SELECT);
        Self::push_filters(&mut qb, &query.filters);
        qb.push(r#" ORDER BY ar."name" ASC, al."title" ASC, t."track_number" ASC"#);
        qb.push(" LIMIT ").push_bind(limit);
        qb.push(" OFFSET ").push_bind(offset);
        let rows: Vec<TrackRow> = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Track::from).collect())
    }

    pub async fn find_page(&self, query: &TracksQuery) -> Result<Page<Track>> {
        let (total, items) = tokio::try_join!(self.count(&query.filters), self.find_many(query))?;
        Ok(Page { total, items })
    }

    pub async fn find(&self, id: i64) -> Result<Track> {
        let query = format!(
            r#"{SELECT} 
            WHERE t."id" = $1"#
        );
        let row: TrackRow = sqlx::query_as(query.as_str())
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.into())
    }

    pub async fn upsert_lidarr(
        &self,
        track: &LidarrTrack,
        track_file: &LidarrTrackFile,
    ) -> Result<Track> {
        let row = sqlx::query_as!(
            IdRow,
            r#"INSERT INTO track (
                "track_number",
                "title",
                "file_path",
                "duration_ms",
                "lidarr_id",
                "musicbrainz_id",
                "album_id",
                "artist_id"
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                (SELECT id FROM album WHERE lidarr_id = $7),
                (SELECT id FROM artist WHERE lidarr_id = $8)
            ) ON CONFLICT(lidarr_id) DO UPDATE SET
                "track_number" = $1,
                "title" = $2,
                "file_path" = $3,
                "duration_ms" = $4,
                "musicbrainz_id" = $6
            RETURNING
                "id""#,
            track.absolute_track_number,
            track.title,
            track_file.path,
            track.duration,
            track.id,
            track.foreign_track_id,
            track.album_id,
            track.artist_id,
        )
        .fetch_one(&self.pool)
        .await?;
        self.find(row.id).await
    }
}
