use anyhow::Result;
use sqlx::SqlitePool;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{
    args::AppArgs,
    services::{
        album::AlbumSerivce, artist::ArtistSerivce, lidarr::LidarrService, lyrics::LyricsService,
        settings::SettingsService, track::TrackService,
    },
    worker::{job::Job, queue::Queue},
};

#[derive(Clone)]
pub struct AppState {
    pub args: AppArgs,

    pub queue: Queue,

    pub album_service: AlbumSerivce,
    pub artist_service: ArtistSerivce,
    pub lidarr_service: LidarrService,
    pub lyrics_service: LyricsService,
    pub settings_service: SettingsService,
    pub track_service: TrackService,
}

impl AppState {
    pub async fn from_args(args: AppArgs) -> Result<Self> {
        let pool = SqlitePool::connect(&args.database_url).await?;

        let settings_service = SettingsService::from_path(&args.settings_path).await?;

        let (queue, receiver) = Queue::with_receiver();

        let state = Self {
            args,
            queue,
            album_service: AlbumSerivce::new(pool.clone()),
            artist_service: ArtistSerivce::new(pool.clone()),
            lidarr_service: LidarrService::new(settings_service.clone()),
            lyrics_service: LyricsService::new(pool.clone()),
            settings_service,
            track_service: TrackService::new(pool.clone()),
        };

        tokio::spawn(state.clone().worker(receiver));

        Ok(state)
    }

    async fn worker(self, mut receiver: UnboundedReceiver<Job>) {
        while let Some(job) = receiver.recv().await {
            if let Err(e) = job.run(&self).await {
                println!("Error while running job: {:}", e);
            }
        }
    }
}
