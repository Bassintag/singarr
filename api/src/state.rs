use std::sync::Arc;

use anyhow::Result;
use sqlx::SqlitePool;

use crate::{
    args::AppArgs,
    services::{
        album::AlbumSerivce, artist::ArtistSerivce, event::EventService, job::JobService,
        jwt::JwtService, lidarr::LidarrService, lyrics::LyricsService, search::SearchService,
        settings::SettingsService, track::TrackService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub args: AppArgs,

    pub album_service: AlbumSerivce,
    pub artist_service: ArtistSerivce,
    pub event_service: EventService,
    pub job_service: JobService,
    pub jwt_service: JwtService,
    pub lidarr_service: LidarrService,
    pub lyrics_service: LyricsService,
    pub search_service: SearchService,
    pub settings_service: SettingsService,
    pub track_service: TrackService,
}

impl AppState {
    pub async fn from_args(args: AppArgs) -> Result<Arc<Self>> {
        let pool = SqlitePool::connect(&args.database_url).await?;

        let settings_service = SettingsService::from_path(&args.settings_path).await?;

        let (job_service, receiver) = JobService::new(pool.clone());

        let state = Self {
            album_service: AlbumSerivce::new(pool.clone()),
            artist_service: ArtistSerivce::new(pool.clone()),
            event_service: EventService::new(),
            job_service,
            jwt_service: JwtService::new(&args)?,
            lidarr_service: LidarrService::new(settings_service.clone()),
            lyrics_service: LyricsService::new(pool.clone(), settings_service.clone()),
            search_service: SearchService::new(pool.clone()),
            settings_service,
            track_service: TrackService::new(pool.clone()),
            args,
        };

        let arc = Arc::new(state);

        tokio::spawn(JobService::start_worker(arc.clone(), receiver));

        Ok(arc)
    }
}
