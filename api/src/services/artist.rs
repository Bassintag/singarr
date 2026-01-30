use std::sync::Arc;

use anyhow::Result;
use sqlx::{prelude::FromRow, Pool, QueryBuilder, Sqlite};

use crate::{
    models::{
        artist::{Artist, ArtistForJob, ArtistWithStats},
        event::Event,
        generic::{IdRow, Page, Pageable, TotalRow, TrackStats},
        lidarr::LidarrArtist,
    },
    services::event::EventService,
};

#[derive(FromRow)]
struct ArtistRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub image_path: Option<String>,
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
                description: value.description,
                image_path: value.image_path,
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
    ar."description",
    ar."image_path",
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
    event_service: Arc<EventService>,
}

impl ArtistSerivce {
    pub fn new(pool: Pool<Sqlite>, event_service: Arc<EventService>) -> Self {
        Self {
            pool,
            event_service,
        }
    }

    pub async fn count(&self) -> Result<i64> {
        let row = sqlx::query_as!(TotalRow, r#"SELECT COUNT(*) as "total" FROM artist"#)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.total)
    }

    pub async fn find_many(&self, pageable_opt: Option<&Pageable>) -> Result<Vec<ArtistWithStats>> {
        let mut qb = QueryBuilder::new(format!(
            r#"{SELECT}
            GROUP BY ar."id" 
            ORDER BY ar."name" ASC "#
        ));
        if let Some(pageable) = pageable_opt {
            pageable.push_limit_offset(&mut qb);
        }
        let rows: Vec<ArtistRow> = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(ArtistWithStats::from).collect())
    }

    pub async fn find_page(&self, pageable: &Pageable) -> Result<Page<ArtistWithStats>> {
        let (total, items) = tokio::try_join!(self.count(), self.find_many(Some(pageable)))?;
        Ok(Page { total, items })
    }

    pub async fn find(&self, id: i64) -> Result<ArtistWithStats> {
        let query = format!(
            r#"{SELECT}
            WHERE ar."id" = $1
            GROUP BY ar."id""#
        );
        let row: ArtistRow = sqlx::query_as(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.into())
    }

    pub async fn find_for_job(&self, id: i64) -> Result<ArtistForJob> {
        let row = sqlx::query_as!(
            ArtistForJob,
            r#"SELECT "id", "lidarr_id", "metadata_updated_at"
            FROM "artist"
            WHERE "id" = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn find_excluding(&self, exclude_ids: &Vec<i64>) -> Result<Vec<IdRow>> {
        let mut qb = sqlx::QueryBuilder::new(
            r#"SELECT "id"
            FROM artist
            WHERE "id" NOT IN "#,
        );
        qb.push_tuples(exclude_ids, |mut qb, id| {
            qb.push_bind(id);
        });
        Ok(qb.build_query_as().fetch_all(&self.pool).await?)
    }

    pub async fn set_metadata(
        &self,
        id: i64,
        image_path: &Option<String>,
        description: &Option<String>,
    ) -> Result<()> {
        sqlx::query_as!(
            IdRow,
            r#"UPDATE artist 
            SET "image_path" = $1, "description" = $2, "metadata_updated_at" = CURRENT_TIMESTAMP
            WHERE id = $3"#,
            image_path,
            description,
            id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
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
        let artist = self.find(row.id).await?;
        self.event_service.send(Event::ArtistCreated { artist });
        Ok(row.id)
    }

    pub async fn remove(&self, id: i64) -> Result<()> {
        sqlx::query!(
            r#"DELETE FROM artist
            WHERE "id" = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;
        self.event_service.send(Event::ArtistDeleted { id });
        Ok(())
    }
}
