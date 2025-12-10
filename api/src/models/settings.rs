use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub root_folder: String,
    pub lidarr: LidarrSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            root_folder: "./".into(),
            lidarr: LidarrSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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
