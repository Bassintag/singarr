use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Result,
    routing, Json, Router,
};

use crate::{http::error::ApiError, models::album::Album, state::AppState};

pub fn routes() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/", routing::get(list))
        .route("/:id", routing::get(get));
}

async fn list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Album>>, ApiError> {
    Ok(Json(state.album_service.find_all().await?))
}

async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Album>, ApiError> {
    Ok(Json(state.album_service.find(id).await?))
}
