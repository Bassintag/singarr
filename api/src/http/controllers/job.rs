use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::Result,
    routing, Json, Router,
};

use crate::{
    http::error::ApiError,
    models::{
        generic::{Page, Pageable},
        job::{Job, JobPayload},
    },
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::get(list))
        .route("/:id", routing::get(get))
        .route("/", routing::post(create))
}

async fn list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<Pageable>,
) -> Result<Json<Page<Job>>, ApiError> {
    Ok(Json(state.job_service.find_page(&query).await?))
}

async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Job>, ApiError> {
    Ok(Json(state.job_service.find(id).await?))
}

async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<JobPayload>,
) -> Result<Json<Job>, ApiError> {
    let job = state.job_service.enqueue(body).await?;
    Ok(Json(job))
}
