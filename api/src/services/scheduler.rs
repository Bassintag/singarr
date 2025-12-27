use std::{slice::Iter, sync::Arc};

use anyhow::Result;
use serde::Serialize;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{models::job::JobPayload, services::job::JobService};

#[derive(Clone, Serialize)]
pub struct ScheduledJob {
    #[serde(skip)]
    pub id: uuid::Uuid,
    pub payload: JobPayload,
    pub cron: String,
}

#[derive(Clone)]
pub struct SchedulerService {
    job_service: Arc<JobService>,
    scheduler: JobScheduler,
    tasks: Vec<ScheduledJob>,
}

impl SchedulerService {
    pub async fn new(job_service: JobService) -> Result<Self> {
        Ok(Self {
            job_service: Arc::new(job_service),
            scheduler: JobScheduler::new().await?,
            tasks: Vec::new(),
        })
    }

    pub async fn start(&self) -> Result<()> {
        Ok(self.scheduler.start().await?)
    }

    pub async fn add_task(&mut self, cron: &str, payload: JobPayload) -> Result<uuid::Uuid> {
        let job_service = self.job_service.clone();

        let payload_clone = payload.clone();
        let job = Job::new_async(cron, move |_uuid, _lock| {
            let job_service = job_service.clone();
            let payload = payload.clone();
            println!("Running task {:?}", payload);
            Box::pin(async move {
                if let Err(e) = job_service.enqueue(payload).await {
                    tracing::error!(error = %e, "Failed to run scheduled job");
                }
            })
        })?;

        let id = self.scheduler.add(job).await?;
        let scheduled = ScheduledJob {
            id: id.clone(),
            payload: payload_clone,
            cron: String::from(cron),
        };
        self.tasks.push(scheduled);
        Ok(id)
    }

    pub async fn add_default_tasks(&mut self) -> Result<()> {
        self.add_task("0 0 * * * *", JobPayload::SyncLibrary)
            .await?;
        self.add_task("0 30 4 * * *", JobPayload::ScanLibrary)
            .await?;
        self.add_task("0 30 */6 * * *", JobPayload::SearchLibrary)
            .await?;

        Ok(())
    }

    pub fn iter_tasks(&'_ self) -> Iter<'_, ScheduledJob> {
        self.tasks.iter()
    }

    pub fn get_tasks(&self) -> Vec<ScheduledJob> {
        self.iter_tasks().map(ScheduledJob::clone).collect()
    }
}
