use anyhow::Result;
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::models::{
    album::Album,
    artist::Artist,
    generic::IdRow,
    lidarr::{LidarrTrack, LidarrTrackFile},
    track::Track,
};

#[derive(FromRow)]
pub struct TrackRow {
    pub id: i64,
    pub title: String,
    pub file_path: String,
    pub duration_ms: Option<i64>,

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
}

impl From<TrackRow> for Track {
    fn from(value: TrackRow) -> Self {
        Self {
            id: value.id,
            title: value.title,
            file_path: value.file_path,
            duration_ms: value.duration_ms.unwrap_or(0),
            artist: Artist {
                id: value.artist_id,
                name: value.artist_name,
                lidarr_id: value.artist_lidarr_id,
                musicbrainz_id: value.artist_musicbrainz_id,
            },
            album: Album {
                id: value.album_id,
                title: value.album_title,
                cover_path: value.album_cover_path,
                lidarr_id: value.album_lidarr_id,
                musicbrainz_id: value.album_musicbrainz_id,
                artist_id: value.album_artist_id,
            },
        }
    }
}

#[derive(Clone)]
pub struct TrackService {
    pool: Pool<Sqlite>,
}

impl TrackService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Track>> {
        let rows = sqlx::query_as!(
            TrackRow,
            r#"SELECT
                t."id",
                t."title",
                t."file_path",
                t."duration_ms",
                ar."id" as "artist_id",
                ar."name" as "artist_name",
                ar."lidarr_id" as "artist_lidarr_id",
                ar."musicbrainz_id" as "artist_musicbrainz_id",
                al."id" as "album_id",
                al."title" as "album_title",
                al."cover_path" as "album_cover_path",
                al."lidarr_id" as "album_lidarr_id",
                al."musicbrainz_id" as "album_musicbrainz_id",
                al."artist_id" as "album_artist_id"
            FROM track t
            INNER JOIN artist ar ON t."artist_id" = ar."id"
            INNER JOIN album al ON t."album_id" = al."id"
            ORDER BY t."title" ASC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(Track::from).collect())
    }

    pub async fn find(&self, id: i64) -> Result<Track> {
        let row = sqlx::query_as!(
            TrackRow,
            r#"SELECT
                t."id",
                t."title",
                t."file_path",
                t."duration_ms",
                ar."id" as "artist_id",
                ar."name" as "artist_name",
                ar."lidarr_id" as "artist_lidarr_id",
                ar."musicbrainz_id" as "artist_musicbrainz_id",
                al."id" as "album_id",
                al."title" as "album_title",
                al."cover_path" as "album_cover_path",
                al."lidarr_id" as "album_lidarr_id",
                al."musicbrainz_id" as "album_musicbrainz_id",
                al."artist_id" as "album_artist_id"
            FROM track t
            INNER JOIN artist ar ON t."artist_id" = ar."id"
            INNER JOIN album al ON t."album_id" = al."id"
            WHERE t."id" = $1"#,
            id
        )
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
