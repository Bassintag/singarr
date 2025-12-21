use std::sync::Arc;

use axum::{extract::State, response::Result, routing, Json, Router};

use crate::{
    http::error::ApiError,
    models::job::{Job, JobPayload},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", routing::post(create))
}

async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<JobPayload>,
) -> Result<Json<Job>, ApiError> {
    let job = state.job_service.enqueue(body).await?;
    Ok(Json(job))
}
