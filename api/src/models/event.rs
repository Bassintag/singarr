use crate::models::{job::Job, lyrics::Lyrics};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    // Job
    #[serde(rename_all = "camelCase")]
    JobStart { job: Job },
    #[serde(rename_all = "camelCase")]
    JobLog { job_id: i64, log: String },
    #[serde(rename_all = "camelCase")]
    JobEnd { job: Job },

    // Lyrics
    #[serde(rename_all = "camelCase")]
    LyricsCreated { lyrics: Lyrics },
    #[serde(rename_all = "camelCase")]
    LyricsDeleted { lyrics: Lyrics },
}
