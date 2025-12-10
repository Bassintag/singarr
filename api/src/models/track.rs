use serde::Serialize;

use crate::models::{album::Album, artist::Artist};

#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub id: i64,
    pub title: String,
    pub file_path: String,
    pub duration_ms: i64,
    pub album: Album,
    pub artist: Artist,
}

impl Track {
    pub fn relative_file_path(&self) -> String {
        if self.file_path.starts_with('/') {
            self.file_path[1..].into()
        } else {
            self.file_path.clone()
        }
    }
}
