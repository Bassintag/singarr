use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LidarrArtistQuery {
    pub mb_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrArtist {
    pub id: i64,
    pub artist_name: String,
    pub foreign_artist_id: Option<String>,
    pub statistics: LidarrArtistStatistics,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrArtistStatistics {
    pub track_file_count: i64,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LidarrAlbumQuery {
    pub artist_id: Option<i64>,
    pub album_ids: Option<Vec<i64>>,
    pub foreign_album_id: Option<String>,
    pub include_all_artist_albums: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrAlbum {
    pub id: i64,
    pub title: String,
    pub foreign_album_id: Option<String>,
    pub artist_id: i64,
    pub duration: i64,
    pub release_date: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LidarrTrackQuery {
    pub artist_id: Option<i64>,
    pub album_id: Option<i64>,
    pub album_release_id: Option<i64>,
    pub track_ids: Option<Vec<i64>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrTrack {
    pub id: i64,
    pub artist_id: i64,
    pub foreign_track_id: Option<String>,
    pub track_file_id: i64,
    pub album_id: i64,
    pub absolute_track_number: i64,
    pub duration: i64,
    pub title: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LidarrTrackFileQuery {
    pub artist_id: Option<i64>,
    pub album_id: Option<i64>,
    pub track_file_id: Option<Vec<i64>>,
    pub unmapped: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrTrackFile {
    pub id: i64,
    pub artist_id: i64,
    pub album_id: i64,
    pub path: String,
    pub size: i64,
}
