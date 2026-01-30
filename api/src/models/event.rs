use crate::models::{
    album::Album, artist::ArtistWithStats, job::Job, lyrics::Lyrics, track::Track,
};
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

    // Artist
    #[serde(rename_all = "camelCase")]
    ArtistCreated { artist: ArtistWithStats },
    #[serde(rename_all = "camelCase")]
    ArtistUpdated { artist: ArtistWithStats },
    #[serde(rename_all = "camelCase")]
    ArtistDeleted { id: i64 },

    // Album
    #[serde(rename_all = "camelCase")]
    AlbumCreated { album: Album },
    #[serde(rename_all = "camelCase")]
    AlbumUpdated { album: Album },
    #[serde(rename_all = "camelCase")]
    AlbumDeleted { id: i64 },

    // Track
    #[serde(rename_all = "camelCase")]
    TrackCreated { track: Track },
    #[serde(rename_all = "camelCase")]
    TrackUpdated { track: Track },
    #[serde(rename_all = "camelCase")]
    TrackDeleted { id: i64 },

    // Lyrics
    #[serde(rename_all = "camelCase")]
    LyricsCreated { lyrics: Lyrics },
    #[serde(rename_all = "camelCase")]
    LyricsDeleted { lyrics: Lyrics },
}
