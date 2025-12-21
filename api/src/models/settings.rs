use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub root_folder: String,
    pub lidarr: LidarrSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            root_folder: "/data".into(),
            lidarr: LidarrSettings::default(),
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
