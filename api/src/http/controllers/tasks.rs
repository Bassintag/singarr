use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};

use crate::{services::scheduler::ScheduledJob, state::AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_all))
}

async fn get_all(State(state): State<Arc<AppState>>) -> Json<Vec<ScheduledJob>> {
    Json(state.scheduler_service.get_tasks())
}
