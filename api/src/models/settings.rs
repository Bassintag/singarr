use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub root_folder: String,
    pub auth: AuthSettings,
    pub lidarr: LidarrSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            root_folder: "/data".into(),
            auth: AuthSettings::default(),
            lidarr: LidarrSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthCredentialsSettings {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AuthSettings {
    pub enabled: bool,
    pub credentials: Option<AuthCredentialsSettings>,
}

impl Default for AuthSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            credentials: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct LidarrSettings {
    pub base_url: String,
    pub http_timeout: u64,
    pub api_key: Option<String>,
}

impl Default for LidarrSettings {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8696/".into(),
            api_key: None,
            http_timeout: 60,
        }
    }
}
