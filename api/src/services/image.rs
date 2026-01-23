use std::path::PathBuf;

use anyhow::Result;

#[derive(Clone)]
pub struct ImageService {
    path: PathBuf,
}

impl ImageService {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn convert(bytes: &Vec<u8>) -> Result<Vec<u8>> {
        let image = image::load_from_memory(&bytes)?.thumbnail(512, 512);
        let webp = webp::Encoder::from_image(&image)
            .map_err(|e| anyhow::Error::msg(String::from(e)))?
            .encode(80.0);
        Ok(webp.to_vec())
    }

    pub async fn resolve_path(&self, relative_path: &PathBuf) -> PathBuf {
        self.path.join("images").join(relative_path)
    }

    pub async fn download(&self, url: &String, relative_path: &PathBuf) -> Result<PathBuf> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        let webp_bytes = Self::convert(&bytes.to_vec())?;
        let output_path = self.resolve_path(relative_path).await;
        if let Some(parent_path) = output_path.parent() {
            tokio::fs::create_dir_all(parent_path).await?;
        }
        tokio::fs::write(&output_path, &webp_bytes).await?;
        Ok(output_path)
    }

    pub async fn remove(&self, relative_path: &PathBuf) -> Result<()> {
        let output_path = self.resolve_path(relative_path).await;
        tokio::fs::remove_file(&output_path).await?;
        Ok(())
    }
}
