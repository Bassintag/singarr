use anyhow::{Ok, Result};
use sqlx::{FromRow, Pool, Sqlite};

use crate::models::search::{Search, SearchAlbum, SearchArtist, SearchResultKind, SearchTrack};

#[derive(FromRow)]
pub struct SearchRow {
    pub kind: String,
    pub image_path: Option<String>,
    pub artist_id: i64,
    pub artist_name: String,
    pub album_id: Option<i64>,
    pub album_title: Option<String>,
    pub track_id: Option<i64>,
    pub track_title: Option<String>,
}

impl TryFrom<SearchRow> for Search {
    type Error = anyhow::Error;

    fn try_from(value: SearchRow) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: SearchResultKind::try_from(value.kind.as_str())?,
            id: value.track_id.or(value.album_id).unwrap_or(value.artist_id),
            image_path: value.image_path,
            artist: Some(SearchArtist {
                id: value.artist_id,
                name: value.artist_name,
            }),
            album: match (value.album_id, value.album_title) {
                (Some(id), Some(title)) => Some(SearchAlbum { id, title }),
                _ => None,
            },
            track: match (value.track_id, value.track_title) {
                (Some(id), Some(title)) => Some(SearchTrack { id, title }),
                _ => None,
            },
        })
    }
}

#[derive(Clone)]
pub struct SearchService {
    pool: Pool<Sqlite>,
}

impl SearchService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, q: &String) -> Result<Vec<Search>> {
        let rows = sqlx::query_as!(
            SearchRow,
            r#"SELECT
                'artist' AS kind,
                artist.image_path AS image_path,
                artist.id AS artist_id,
                artist.name AS artist_name,
                NULL AS album_id,
                NULL AS album_title,
                NULL AS track_id,
                NULL AS track_title
            FROM artist
            WHERE artist.name LIKE '%' || $1 || '%' COLLATE NOCASE

            UNION ALL

            SELECT
                'album' AS kind,
                album.cover_path AS image_path,
                artist.id AS artist_id,
                artist.name AS artist_name,
                album.id AS album_id,
                album.title AS album_title,
                NULL AS track_id,
                NULL AS track_title
            FROM album
            JOIN artist ON artist.id = album.artist_id
            WHERE album.title LIKE '%' || $1 || '%' COLLATE NOCASE

            UNION ALL

            SELECT
                'track' AS kind,
                album.cover_path AS image_path,
                artist.id AS artist_id,
                artist.name AS artist_name,
                album.id AS album_id,
                album.title AS album_title,
                track.id AS track_id,
                track.title AS track_title
            FROM track
            JOIN album  ON album.id = track.album_id
            JOIN artist ON artist.id = album.artist_id
            WHERE track.title LIKE '%' || $1 || '%' COLLATE NOCASE

            LIMIT 10"#,
            q
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(Search::try_from)
            .collect::<Result<_, _>>()?)
    }
}
