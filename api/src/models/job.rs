use core::fmt;
use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::event::Event,
    state::AppState,
    worker::jobs::{
        clean_album::{clean_album, CleanAlbumParams},
        import_lyrics::{import_lyrics, ImportLyricsParams},
        scan_album::{scan_album, ScanAlbumParams},
        scan_artist::{scan_artist, ScanArtistParams},
        scan_track::{scan_track, ScanTrackParams},
        search_album::{search_album, SearchAlbumParams},
        search_artist::{search_artist, SearchArtistParams},
        search_track::{search_track, SearchTrackParams},
        sync_artist::{sync_artist, SyncArtistParams},
        sync_library::sync_library,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum JobPayload {
    // Clean
    CleanAlbum(CleanAlbumParams),

    // Import
    ImportLyrics(ImportLyricsParams),

    // Scan
    ScanArtist(ScanArtistParams),
    ScanAlbum(ScanAlbumParams),
    ScanTrack(ScanTrackParams),

    // Search
    SearchArtist(SearchArtistParams),
    SearchAlbum(SearchAlbumParams),
    SearchTrack(SearchTrackParams),

    // Sync
    SyncLibrary,
    SyncArtist(SyncArtistParams),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JobStatus {
    Pending,
    Running,
    Done,
    Failed,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            JobStatus::Pending => "pending",
            JobStatus::Running => "running",
            JobStatus::Done => "done",
            JobStatus::Failed => "failed",
        };
        f.write_str(s)
    }
}

impl FromStr for JobStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(JobStatus::Pending),
            "running" => Ok(JobStatus::Running),
            "done" => Ok(JobStatus::Done),
            "failed" => Ok(JobStatus::Failed),
            _ => Err(anyhow::anyhow!("invalid job status")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: i64,
    pub payload: JobPayload,
    pub status: JobStatus,
    pub error: Option<String>,
}

impl Job {
    async fn dispatch<T: Clone, F, Fut>(
        &self,
        state: &Arc<AppState>,
        params: &T,
        f: F,
    ) -> Result<()>
    where
        F: FnOnce(JobContext<T>) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let context = JobContext {
            job_id: self.id,
            state: state.clone(),
            params: params.clone(),
        };
        f(context).await
    }

    pub async fn run(&self, state: &Arc<AppState>) -> Result<()> {
        match &self.payload {
            JobPayload::CleanAlbum(p) => self.dispatch(state, p, clean_album).await,

            JobPayload::ImportLyrics(p) => self.dispatch(state, p, import_lyrics).await,

            JobPayload::ScanArtist(p) => self.dispatch(state, p, scan_artist).await,
            JobPayload::ScanAlbum(p) => self.dispatch(state, p, scan_album).await,
            JobPayload::ScanTrack(p) => self.dispatch(state, p, scan_track).await,

            JobPayload::SearchArtist(p) => self.dispatch(state, p, search_artist).await,
            JobPayload::SearchAlbum(p) => self.dispatch(state, p, search_album).await,
            JobPayload::SearchTrack(p) => self.dispatch(state, p, search_track).await,

            JobPayload::SyncLibrary => self.dispatch(state, &(), sync_library).await,

            JobPayload::SyncArtist(p) => self.dispatch(state, p, sync_artist).await,
        }
    }
}

pub struct JobContext<T> {
    pub job_id: i64,
    pub state: Arc<AppState>,
    pub params: T,
}

impl<T> JobContext<T> {
    pub fn log<S: Into<String>>(&self, log: S) {
        let log_s = log.into();
        println!("[job #{}] {}", self.job_id, &log_s);
        let _ = self.state.event_service.send(Event::JobLog {
            job_id: self.job_id,
            log: log_s,
        });
    }

    pub fn clone_with_params<U>(&self, params: U) -> JobContext<U> {
        JobContext {
            job_id: self.job_id,
            state: self.state.clone(),
            params,
        }
    }
}
