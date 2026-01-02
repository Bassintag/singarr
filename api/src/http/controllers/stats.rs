use std::sync::Arc;

use axum::{extract::State, routing, Json, Router};

use crate::{http::error::ApiError, models::stats::CountsStats, state::AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/counts", routing::get(get))
}

pub async fn get(State(state): State<Arc<AppState>>) -> Result<Json<CountsStats>, ApiError> {
    let stats = state.stats_service.get_counts().await?;
    Ok(Json(stats))
}
