use anyhow::Result;
use reqwest::{Client, Method, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    models::lidarr::{
        LidarrAlbum, LidarrAlbumQuery, LidarrArtist, LidarrArtistQuery, LidarrTrack,
        LidarrTrackFile, LidarrTrackFileQuery, LidarrTrackQuery,
    },
    services::settings::SettingsService,
};

#[derive(Clone)]
pub struct LidarrService {
    settings_service: SettingsService,
    client: Client,
}

impl LidarrService {
    pub fn new(settings_service: SettingsService) -> Self {
        Self {
            settings_service,
            client: Client::new(),
        }
    }

    pub async fn request<T: DeserializeOwned, Q: Serialize>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
    ) -> Result<T> {
        let settings = self.settings_service.get().await;
        let mut url = Url::parse(&settings.lidarr.base_url).unwrap();
        url.set_path(path);
        if let Some(q) = query {
            let query_string = serde_qs::to_string(q)?;
            url.set_query(Some(&query_string));
        }
        println!("[LIDARR] Fetch '{}'", url);
        let mut builder = self.client.request(method, url);
        if let Some(api_key) = settings.lidarr.api_key {
            builder = builder.header("X-Api-Key", api_key)
        }
        Ok(builder.send().await?.json::<T>().await?)
    }

    pub async fn list_artists(
        &self,
        query: Option<&LidarrArtistQuery>,
    ) -> Result<Vec<LidarrArtist>> {
        self.request(Method::GET, "api/v1/artist", query).await
    }

    pub async fn list_albums(&self, query: Option<&LidarrAlbumQuery>) -> Result<Vec<LidarrAlbum>> {
        self.request(Method::GET, "api/v1/album", query).await
    }

    pub async fn list_tracks(&self, query: Option<&LidarrTrackQuery>) -> Result<Vec<LidarrTrack>> {
        self.request(Method::GET, "api/v1/track", query).await
    }

    pub async fn list_track_files(
        &self,
        query: Option<&LidarrTrackFileQuery>,
    ) -> Result<Vec<LidarrTrackFile>> {
        self.request(Method::GET, "api/v1/trackfile", query).await
    }
}
