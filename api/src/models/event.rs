use crate::models::job::Job;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    #[serde(rename_all = "camelCase")]
    JobStart { job: Job },
    #[serde(rename_all = "camelCase")]
    JobLog { job_id: i64, log: String },
    #[serde(rename_all = "camelCase")]
    JobEnd { job: Job },
}
