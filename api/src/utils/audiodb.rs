use anyhow::Result;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudiodbArtist {
    pub str_artist_thumb: Option<String>,
    #[serde(rename = "strBiographyEN")]
    pub str_biography_en: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudiodbAlbum {
    pub str_album_thumb: Option<String>,
    #[serde(rename = "strDescriptionEN")]
    pub str_description_en: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudiodbArtistResponse {
    pub artists: Option<Vec<AudiodbArtist>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudiodbAlbumResponse {
    pub album: Option<Vec<AudiodbAlbum>>,
}

pub struct AudiodbClient {
    base_url: String,
    client: Client,
}

impl AudiodbClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://www.theaudiodb.com/api/v1/json/123/".into(),
            client: Client::new(),
        }
    }

    pub async fn request<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        println!("[AUDIODB] Fetch '{}'", url);
        let builder = self.client.get(url);
        Ok(builder.send().await?.json::<T>().await?)
    }

    pub async fn lookup_album(&self, mb_album_id: &String) -> Result<AudiodbAlbumResponse> {
        self.request(format!("album-mb.php?i={}", mb_album_id).as_str())
            .await
    }

    pub async fn lookup_artist(&self, mb_artist_id: &String) -> Result<AudiodbArtistResponse> {
        self.request(format!("artist-mb.php?i={}", mb_artist_id).as_str())
            .await
    }
}
