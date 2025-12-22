use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::Result,
    routing, Json, Router,
};

use crate::{
    http::error::ApiError,
    models::{
        artist::ArtistWithStats,
        generic::{Page, Pageable},
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
    Query(query): Query<Pageable>,
) -> Result<Json<Page<ArtistWithStats>>, ApiError> {
    Ok(Json(state.artist_service.find_page(&query).await?))
}

async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<ArtistWithStats>, ApiError> {
    Ok(Json(state.artist_service.find(id).await?))
}
