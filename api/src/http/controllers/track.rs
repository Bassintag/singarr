use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Result,
    routing, Json, Router,
};

use crate::{http::error::ApiError, models::track::Track, state::AppState};

pub fn routes() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/", routing::get(list))
        .route("/:id", routing::get(get));
}

async fn list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Track>>, ApiError> {
    Ok(Json(state.track_service.find_all().await?))
}

async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Track>, ApiError> {
    Ok(Json(state.track_service.find(id).await?))
}
