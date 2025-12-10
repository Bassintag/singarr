use anyhow::Result;

use crate::{
    state::AppState,
    worker::jobs::{
        import_lyrics::{import_lyrics, ImportLyricsParams},
        search_track::{search_track, SearchTrackParams},
        sync_library::sync_library,
        sync_track::{sync_track, SyncTrackParams},
    },
};

#[derive(Debug)]
pub enum Job {
    ImportLyrics(ImportLyricsParams),

    SearchTrack(SearchTrackParams),

    SyncLibrary,
    SyncTrack(SyncTrackParams),
}

impl Job {
    pub async fn run(&self, state: &AppState) -> Result<()> {
        match self {
            Job::ImportLyrics(params) => import_lyrics(state, params).await,

            Job::SearchTrack(params) => search_track(state, params).await,

            Job::SyncLibrary => sync_library(state).await,
            Job::SyncTrack(params) => sync_track(state, params).await,
        }
    }
}
