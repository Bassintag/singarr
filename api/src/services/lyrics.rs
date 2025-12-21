use std::path::PathBuf;

use anyhow::Result;
use sqlx::{prelude::FromRow, Pool, QueryBuilder, Sqlite};

use crate::{
    models::{
        generic::{IdRow, Page, TotalRow},
        lyrics::{CreateLyrics, Lyrics, LyricsContent, LyricsFilters, LyricsQuery},
    },
    services::settings::SettingsService,
    utils::{
        checksum::md5sum,
        lrc::{LrcParser, LyricsType},
    },
};

#[derive(FromRow)]
pub struct LyricsPathRow {
    file_path: String,
}

const SELECT: &str = r#"SELECT 
    "id", 
    "synced", 
    "file_path", 
    "checksum" 
FROM lyrics l"#;

#[derive(Clone)]
pub struct LyricsService {
    pool: Pool<Sqlite>,
    settings_service: SettingsService,
}

impl LyricsService {
    pub fn new(pool: Pool<Sqlite>, settings_service: SettingsService) -> Self {
        Self {
            pool,
            settings_service,
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
        let rows = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows)
    }

    pub async fn find_many(&self, query: &LyricsQuery) -> Result<Vec<Lyrics>> {
        let (limit, offset) = query.pageable.to_limit_offset();
        let mut qb = QueryBuilder::new(SELECT);
        Self::push_filters(&mut qb, &query.filters);
        qb.push(r#" ORDER BY "file_path" ASC"#);
        qb.push(" LIMIT ").push_bind(limit);
        qb.push(" OFFSET ").push_bind(offset);
        let rows: Vec<Lyrics> = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows)
    }

    pub async fn find_page(&self, query: &LyricsQuery) -> Result<Page<Lyrics>> {
        let (total, items) = tokio::try_join!(self.count(&query.filters), self.find_many(query))?;
        Ok(Page { total, items })
    }

    pub async fn find(&self, id: i64) -> Result<Lyrics> {
        let query = format!(
            r#"{SELECT}
            WHERE "id" = $1"#
        );
        let row = sqlx::query_as(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row)
    }

    pub async fn find_by_path(&self, path: &String) -> Result<Option<Lyrics>> {
        let query = format!(
            r#"{SELECT}
            WHERE "file_path" = $1"#
        );
        let row = sqlx::query_as(&query)
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
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
        Ok(row.id)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let path = self.resolve_path(id).await?;
        sqlx::query!(
            r#"DELETE FROM lyrics
            WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;
        tokio::fs::remove_file(&path).await?;
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
