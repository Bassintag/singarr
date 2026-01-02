use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use sqlx::{prelude::FromRow, Pool, QueryBuilder, Sqlite};

use crate::{
    models::{
        album::{Album, AlbumWithArtist},
        artist::Artist,
        event::Event,
        generic::{IdRow, Page, TotalRow},
        lyrics::{CreateLyrics, Lyrics, LyricsContent, LyricsFilters, LyricsQuery},
        track::Track,
    },
    services::{event::EventService, settings::SettingsService},
    utils::{
        checksum::md5sum,
        lrc::{LrcParser, LyricsType},
    },
};

#[derive(FromRow)]
struct LyricsRow {
    pub id: i64,
    pub synced: bool,
    pub file_path: String,
    pub checksum: String,
    pub provider: Option<String>,

    pub track_id: i64,
    pub track_track_number: Option<i64>,
    pub track_title: String,
    pub track_file_path: String,
    pub track_duration_ms: Option<i64>,
    pub track_has_lyrics: bool,

    pub track_artist_id: i64,
    pub track_artist_name: String,
    pub track_artist_description: Option<String>,
    pub track_artist_image_path: Option<String>,
    pub track_artist_lidarr_id: Option<i64>,
    pub track_artist_musicbrainz_id: Option<String>,

    pub track_album_id: i64,
    pub track_album_title: String,
    pub track_album_description: Option<String>,
    pub track_album_cover_path: Option<String>,
    pub track_album_lidarr_id: Option<i64>,
    pub track_album_musicbrainz_id: Option<String>,

    pub track_album_artist_id: i64,
    pub track_album_artist_name: String,
    pub track_album_artist_description: Option<String>,
    pub track_album_artist_image_path: Option<String>,
    pub track_album_artist_lidarr_id: Option<i64>,
    pub track_album_artist_musicbrainz_id: Option<String>,
}

impl From<LyricsRow> for Lyrics {
    fn from(value: LyricsRow) -> Self {
        Self {
            id: value.id,
            synced: value.synced,
            file_path: value.file_path,
            checksum: value.checksum,
            provider: value.provider,
            track: Track {
                id: value.track_id,
                track_number: value.track_track_number.unwrap_or(0),
                title: value.track_title,
                file_path: value.track_file_path,
                duration_ms: value.track_duration_ms.unwrap_or(0),
                has_lyrics: value.track_has_lyrics,
                artist: Artist {
                    id: value.track_artist_id,
                    name: value.track_artist_name,
                    description: value.track_artist_description,
                    image_path: value.track_artist_image_path,
                    lidarr_id: value.track_artist_lidarr_id,
                    musicbrainz_id: value.track_artist_musicbrainz_id,
                },
                album: AlbumWithArtist {
                    album: Album {
                        id: value.track_album_id,
                        title: value.track_album_title,
                        description: value.track_album_description,
                        cover_path: value.track_album_cover_path,
                        lidarr_id: value.track_album_lidarr_id,
                        musicbrainz_id: value.track_album_musicbrainz_id,
                    },
                    artist: Artist {
                        id: value.track_album_artist_id,
                        name: value.track_album_artist_name,
                        description: value.track_album_artist_description,
                        image_path: value.track_album_artist_image_path,
                        lidarr_id: value.track_album_artist_lidarr_id,
                        musicbrainz_id: value.track_album_artist_musicbrainz_id,
                    },
                },
            },
        }
    }
}

#[derive(FromRow)]
pub struct LyricsPathRow {
    file_path: String,
}

const SELECT: &str = r#"SELECT 
    l."id",
    l."synced", 
    l."file_path", 
    l."checksum",
    l."provider",

    t."id" as "track_id",
    t."track_number" as "track_track_number",
    t."title" as "track_title",
    t."file_path" as "track_file_path",
    t."duration_ms" as "track_duration_ms",
    EXISTS (SELECT 1 FROM lyrics l WHERE l."track_id" = t."id") AS "track_has_lyrics",

    ar."id" as "track_artist_id",
    ar."name" as "track_artist_name",
    ar."description" as "track_artist_description",
    ar."image_path" as "track_artist_image_path",
    ar."lidarr_id" as "track_artist_lidarr_id",
    ar."musicbrainz_id" as "track_artist_musicbrainz_id",

    al."id" as "track_album_id",
    al."title" as "track_album_title",
    al."description" as "track_album_description",
    al."cover_path" as "track_album_cover_path",
    al."lidarr_id" as "track_album_lidarr_id",
    al."musicbrainz_id" as "track_album_musicbrainz_id",

    al_ar."id" as "track_album_artist_id",
    al_ar."name" as "track_album_artist_name",
    al_ar."description" as "track_album_artist_description",
    al_ar."image_path" as "track_album_artist_image_path",
    al_ar."lidarr_id" as "track_album_artist_lidarr_id",
    al_ar."musicbrainz_id" as "track_album_artist_musicbrainz_id"
FROM lyrics l
INNER JOIN track t ON l."track_id" = t."id"
INNER JOIN artist ar ON t."artist_id" = ar."id"
INNER JOIN album al ON t."album_id" = al."id"
INNER JOIN artist al_ar ON al."artist_id" = al_ar."id""#;

#[derive(Clone)]
pub struct LyricsService {
    pool: Pool<Sqlite>,
    settings_service: Arc<SettingsService>,
    event_service: Arc<EventService>,
}

impl LyricsService {
    pub fn new(
        pool: Pool<Sqlite>,
        settings_service: Arc<SettingsService>,
        event_service: Arc<EventService>,
    ) -> Self {
        Self {
            pool,
            settings_service,
            event_service,
        }
    }

    async fn resolve_path(&self, track_id: i64) -> Result<PathBuf> {
        let row = sqlx::query_as!(
            LyricsPathRow,
            r#"SELECT l."file_path" FROM lyrics l WHERE id = $1"#,
            track_id
        )
        .fetch_one(&self.pool)
        .await?;

        let root_folder = self.settings_service.get().await.root_folder;

        let path = PathBuf::from(&root_folder).join(&row.file_path);

        Ok(path)
    }

    fn push_filters(qb: &mut QueryBuilder<'_, Sqlite>, filters: &LyricsFilters) {
        qb.push(" WHERE 1=1");
        if let Some(artist_id) = filters.artist_id {
            qb.push(
                r#" AND EXISTS (
            SELECT 1 FROM track t
            WHERE t."id" = l."track_id"
            AND t."artist_id" = "#,
            )
            .push_bind(artist_id)
            .push(")");
        }
        if let Some(album_id) = filters.album_id {
            qb.push(
                r#" AND EXISTS (
            SELECT 1 FROM track t
            WHERE t."id" = l."track_id"
            AND t."album_id" = "#,
            )
            .push_bind(album_id)
            .push(")");
        }
        if let Some(track_id) = filters.track_id {
            qb.push(r#" AND l."track_id" = "#).push_bind(track_id);
        }
    }

    pub async fn count(&self, filters: &LyricsFilters) -> Result<i64> {
        let mut qb = sqlx::QueryBuilder::new(
            r#"SELECT
                COUNT(*) as "total"
            FROM lyrics l"#,
        );
        Self::push_filters(&mut qb, filters);
        let row: TotalRow = qb.build_query_as().fetch_one(&self.pool).await?;
        Ok(row.total)
    }

    pub async fn find_all(&self, filters: &LyricsFilters) -> Result<Vec<Lyrics>> {
        let mut qb = QueryBuilder::new(SELECT);
        Self::push_filters(&mut qb, filters);
        qb.push(r#" ORDER BY "created_at" ASC"#);
        let rows: Vec<LyricsRow> = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Lyrics::from).collect())
    }

    pub async fn find_many(&self, query: &LyricsQuery) -> Result<Vec<Lyrics>> {
        let mut qb = QueryBuilder::new(SELECT);
        Self::push_filters(&mut qb, &query.filters);
        qb.push(r#" ORDER BY l."file_path" ASC"#);
        if let Some(pageable) = &query.pageable {
            pageable.push_limit_offset(&mut qb);
        }
        let rows: Vec<LyricsRow> = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Lyrics::from).collect())
    }

    pub async fn find_page(&self, query: &LyricsQuery) -> Result<Page<Lyrics>> {
        let (total, items) = tokio::try_join!(self.count(&query.filters), self.find_many(query))?;
        Ok(Page { total, items })
    }

    pub async fn find(&self, id: i64) -> Result<Lyrics> {
        let query = format!(
            r#"{SELECT}
            WHERE l."id" = $1"#
        );
        let row: LyricsRow = sqlx::query_as(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.into())
    }

    pub async fn find_by_path(&self, path: &String) -> Result<Option<Lyrics>> {
        let query = format!(
            r#"{SELECT}
            WHERE l."file_path" = $1"#
        );
        let row: Option<LyricsRow> = sqlx::query_as(&query)
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Lyrics::from))
    }

    pub async fn create(&self, data: &CreateLyrics) -> Result<i64> {
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
        let lyrics = self.find(row.id).await?;
        self.event_service.send(Event::LyricsCreated { lyrics })?;
        Ok(row.id)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let path = self.resolve_path(id).await?;
        let lyrics = self.find(id).await?;
        sqlx::query!(
            r#"DELETE FROM lyrics
            WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;
        tokio::fs::remove_file(&path).await?;
        self.event_service.send(Event::LyricsDeleted { lyrics })?;
        Ok(())
    }

    pub async fn delete_many(&self, album_id: i64, exclude_ids: &Vec<i64>) -> Result<()> {
        let mut qb = sqlx::QueryBuilder::new(
            "DELETE FROM lyrics 
            WHERE id NOT IN",
        );
        qb.push_tuples(exclude_ids, |mut qb, id| {
            qb.push_bind(id);
        });
        qb.push(
            r#" AND EXISTS (
            SELECT 1 FROM track t
            WHERE t."id" = lyrics."track_id"
            AND t."album_id" = "#,
        )
        .push_bind(album_id)
        .push(")");
        qb.build().fetch_all(&self.pool).await?;
        Ok(())
    }

    pub async fn create_from_path(&self, track_id: i64, path: &PathBuf) -> Result<i64> {
        let settings = self.settings_service.get().await;
        let relative_path = path.strip_prefix(settings.root_folder)?;
        let content = tokio::fs::read_to_string(path).await?;
        let checksum = md5sum(&content);
        let lrc = LrcParser::new(content).parse();
        let data = CreateLyrics {
            checksum,
            track_id,
            file_path: relative_path.to_string_lossy().into(),
            synced: lrc.lyrics_type() == LyricsType::Synced,
            language: None,
            provider: None,
        };
        Ok(self.create(&data).await?)
    }

    pub async fn get_content(&self, id: i64) -> Result<LyricsContent> {
        let path = self.resolve_path(id).await?;
        let text = tokio::fs::read_to_string(&path).await?;

        Ok(LyricsContent { text })
    }
}
