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
        lyrics::{Lyrics, LyricsContent, LyricsQuery},
    },
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/", routing::get(list))
        .route("/:id", routing::get(get))
        .route("/:id", routing::delete(delete))
        .route("/:id/content", routing::get(get_content));
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LyricsQuery>,
) -> Result<Json<Page<Lyrics>>, ApiError> {
    Ok(Json(state.lyrics_service.find_page(&query).await?))
}

pub async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Lyrics>, ApiError> {
    Ok(Json(state.lyrics_service.find(id).await?))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<(), ApiError> {
    Ok(state.lyrics_service.delete(id).await?)
}

pub async fn get_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<LyricsContent>, ApiError> {
    Ok(Json(state.lyrics_service.get_content(id).await?))
}
