use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::Result,
    routing, Json, Router,
};

use crate::{
    http::error::ApiError,
    models::{
        album::{AlbumWithStats, AlbumsQuery},
        generic::Page,
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
    Query(query): Query<AlbumsQuery>,
) -> Result<Json<Page<AlbumWithStats>>, ApiError> {
    Ok(Json(state.album_service.find_page(&query).await?))
}

async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<AlbumWithStats>, ApiError> {
    Ok(Json(state.album_service.find(id).await?))
}
