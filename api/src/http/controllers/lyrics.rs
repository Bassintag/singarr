use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Result,
    routing, Json, Router,
};

use crate::{http::error::ApiError, models::lyrics::Lyrics, state::AppState};

pub fn routes() -> Router<Arc<AppState>> {
    return Router::new().route("/", routing::get(list));
}

pub async fn list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Lyrics>>, ApiError> {
    Ok(Json(state.lyrics_service.find_all().await?))
}

pub async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Lyrics>, ApiError> {
    Ok(Json(state.lyrics_service.find(id).await?))
}
