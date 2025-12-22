use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::Result,
    routing, Json, Router,
};

use crate::{
    http::error::ApiError,
    models::{
        generic::Page,
        track::{Track, TracksQuery},
    },
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::get(list))
        .route("/:id", routing::get(get))
}

async fn list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TracksQuery>,
) -> Result<Json<Page<Track>>, ApiError> {
    Ok(Json(state.track_service.find_page(&query).await?))
}

async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Track>, ApiError> {
    Ok(Json(state.track_service.find(id).await?))
}
