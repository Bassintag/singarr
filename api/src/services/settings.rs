use std::{path::PathBuf, sync::Arc};

use anyhow::{Ok, Result};
use tokio::sync::RwLock;

use crate::models::settings::Settings;

#[derive(Clone)]
pub struct SettingsService {
    path: PathBuf,
    state: Arc<RwLock<Settings>>,
}

impl SettingsService {
    pub async fn from_path(path: impl Into<PathBuf>) -> Result<Self> {
        let path_buf: PathBuf = path.into();
        let settings = if path_buf.exists() {
            let data = tokio::fs::read_to_string(&path_buf).await?;
            serde_json::from_str::<Settings>(&data)?
        } else {
            Settings::default()
        };

        Ok(Self {
            path: path_buf,
            state: Arc::new(RwLock::new(settings)),
        })
    }

    pub async fn get(&self) -> Settings {
        self.state.read().await.clone()
    }

    pub async fn set(&self, value: Settings) -> Result<()> {
        {
            let mut settings = self.state.write().await;
            *settings = value;
        }
        self.save().await?;
        Ok(())
    }

    async fn save(&self) -> Result<()> {
        let settings = self.state.read().await;
        let data = serde_json::to_string_pretty(&*settings)?;
        tokio::fs::write(&self.path, data).await?;
        Ok(())
    }
}
