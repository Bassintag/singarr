use std::sync::Arc;

use anyhow::Result;
use sqlx::{prelude::FromRow, Pool, QueryBuilder, Sqlite};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{
    models::{
        event::Event,
        generic::{IdRow, Page, Pageable, TotalRow},
        job::{Job, JobPayload, JobStatus},
    },
    state::AppState,
};

#[derive(FromRow)]
pub struct JobRow {
    id: i64,
    created_at: String,
    payload: String,
    status: String,
    error: Option<String>,
}

impl TryFrom<JobRow> for Job {
    type Error = anyhow::Error;

    fn try_from(value: JobRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            created_at: value.created_at,
            payload: serde_json::from_str(&value.payload)?,
            status: value.status.parse()?,
            error: value.error,
        })
    }
}

const SELECT: &str = r#"SELECT
    j."id",
    j."created_at",
    j."payload",
    j."status",
    j."error"
FROM job j"#;

#[derive(Clone)]
pub struct JobService {
    pool: Pool<Sqlite>,
    sender: UnboundedSender<Job>,
}

impl JobService {
    pub fn new(pool: Pool<Sqlite>) -> (Self, UnboundedReceiver<Job>) {
        let (sender, reciever) = mpsc::unbounded_channel();
        (Self { pool, sender }, reciever)
    }

    pub async fn start_worker(state: Arc<AppState>, mut receiver: UnboundedReceiver<Job>) {
        while let Some(mut job) = receiver.recv().await {
            state
                .job_service
                .update_status(job.id, JobStatus::Running)
                .await
                .ok();
            job.status = JobStatus::Running;
            state
                .event_service
                .send(Event::JobStart { job: job.clone() })
                .ok();
            if let Err(e) = job.run(&state).await {
                println!("Error while running job: {:}", e);
                state
                    .job_service
                    .update_failed(job.id, e.to_string())
                    .await
                    .ok();
            } else {
                state.job_service.update_done(job.id).await.ok();
            }
            if let Ok(job) = state.job_service.find(job.id).await {
                state.event_service.send(Event::JobEnd { job }).ok();
            }
        }
    }

    pub async fn count(&self) -> Result<i64> {
        let row = sqlx::query_as!(TotalRow, r#"SELECT COUNT(*) as "total" FROM artist"#)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.total)
    }

    pub async fn find_many(&self, pageable_opt: Option<&Pageable>) -> Result<Vec<Job>> {
        let mut qb = QueryBuilder::new(format!(
            r#"{SELECT}
            ORDER BY j."created_at" DESC "#
        ));
        if let Some(pageable) = pageable_opt {
            pageable.push_limit_offset(&mut qb);
        }
        let rows: Vec<JobRow> = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows
            .into_iter()
            .map(Job::try_from)
            .collect::<Result<_, _>>()?)
    }

    pub async fn find_page(&self, pageable: &Pageable) -> Result<Page<Job>> {
        let (total, items) = tokio::try_join!(self.count(), self.find_many(Some(pageable)))?;
        Ok(Page { total, items })
    }

    pub async fn find(&self, id: i64) -> Result<Job> {
        let query = format!(
            r#"{SELECT}
            WHERE j."id" = $1"#
        );
        let row: JobRow = sqlx::query_as(query.as_str())
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        row.try_into()
    }

    pub async fn create(&self, payload: &JobPayload) -> Result<i64> {
        let payload_string = serde_json::to_string(payload)?;
        let status_string = JobStatus::Pending.to_string();
        let row = sqlx::query_as!(
            IdRow,
            r#"INSERT INTO job (
                "payload",
                "status"
            ) VALUES (
                $1, $2
            ) RETURNING "id""#,
            payload_string,
            status_string,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.id)
    }

    pub async fn update_status(&self, id: i64, status: JobStatus) -> Result<()> {
        let status_string = status.to_string();
        sqlx::query!(
            r#"UPDATE job
            SET "status" = $1
            WHERE "id" = $2"#,
            status_string,
            id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_done(&self, id: i64) -> Result<()> {
        let status_string = JobStatus::Done.to_string();
        sqlx::query!(
            r#"UPDATE job
            SET 
                "status" = $1,
                "completed_at" = CURRENT_TIMESTAMP
            WHERE "id" = $2"#,
            status_string,
            id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_failed(&self, id: i64, error: String) -> Result<()> {
        let status_string = JobStatus::Failed.to_string();
        sqlx::query!(
            r#"UPDATE job
            SET 
                "status" = $1,
                "error" = $2,
                "completed_at" = CURRENT_TIMESTAMP
            WHERE "id" = $3"#,
            status_string,
            error,
            id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn enqueue(&self, payload: JobPayload) -> Result<Job> {
        let id = self.create(&payload).await?;
        let job = self.find(id).await?;
        self.sender.send(job.clone())?;
        Ok(job)
    }
}
