use std::sync::Arc;

use anyhow::Result;
use sqlx::SqlitePool;

use crate::{
    args::AppArgs,
    services::{
        album::AlbumSerivce, artist::ArtistSerivce, event::EventService, image::ImageService,
        job::JobService, jwt::JwtService, lidarr::LidarrService, lyrics::LyricsService,
        notifier::NotifierService, scheduler::SchedulerService, search::SearchService,
        settings::SettingsService, stats::StatsService, track::TrackService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub args: Arc<AppArgs>,

    pub album_service: Arc<AlbumSerivce>,
    pub artist_service: Arc<ArtistSerivce>,
    pub event_service: Arc<EventService>,
    pub image_service: Arc<ImageService>,
    pub job_service: Arc<JobService>,
    pub jwt_service: Arc<JwtService>,
    pub lidarr_service: Arc<LidarrService>,
    pub lyrics_service: Arc<LyricsService>,
    pub notifier_service: Arc<NotifierService>,
    pub scheduler_service: Arc<SchedulerService>,
    pub search_service: Arc<SearchService>,
    pub settings_service: Arc<SettingsService>,
    pub stats_service: Arc<StatsService>,
    pub track_service: Arc<TrackService>,
}

impl AppState {
    pub async fn from_args(args: AppArgs) -> Result<Arc<Self>> {
        let args = Arc::new(args);
        let pool = SqlitePool::connect(&args.database_url).await?;

        let settings_service = Arc::new(SettingsService::from_path(&args.settings_path).await?);

        let (job_service, receiver) = JobService::new(pool.clone());
        let job_service = Arc::new(job_service);

        let mut scheduler_service = SchedulerService::new(job_service.clone()).await?;
        scheduler_service.add_default_tasks().await?;
        let scheduler_service = Arc::new(scheduler_service);

        let event_service = Arc::new(EventService::new());
        let album_service = Arc::new(AlbumSerivce::new(pool.clone()));
        let artist_service = Arc::new(ArtistSerivce::new(pool.clone()));
        let track_service = Arc::new(TrackService::new(pool.clone()));
        let lyrics_service = Arc::new(LyricsService::new(
            pool.clone(),
            settings_service.clone(),
            event_service.clone(),
        ));
        let search_service = Arc::new(SearchService::new(pool.clone()));
        let jwt_service = Arc::new(JwtService::new(&args)?);
        let lidarr_service = Arc::new(LidarrService::new(settings_service.clone()));
        let image_service = Arc::new(ImageService::new(settings_service.clone()));
        let notifier_service = Arc::new(NotifierService::new(pool.clone()));
        let stats_service = Arc::new(StatsService::new(pool.clone()));

        let state = Arc::new(Self {
            album_service,
            artist_service,
            event_service,
            image_service,
            job_service,
            jwt_service,
            lidarr_service,
            lyrics_service,
            notifier_service,
            scheduler_service,
            search_service,
            settings_service,
            stats_service,
            track_service,
            args,
        });

        {
            let arc = state.clone();
            tokio::spawn(JobService::start_worker(arc, receiver));
        }
        {
            let arc = state.clone();
            let notifier_service = arc.notifier_service.clone();
            tokio::spawn(async move { notifier_service.start_worker(arc).await });
        }

        state.scheduler_service.start().await?;

        Ok(state)
    }
}
