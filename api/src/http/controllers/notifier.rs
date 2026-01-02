use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing, Json, Router,
};

use crate::{
    http::error::ApiError,
    models::notifier::{CreateNotifier, Notifier},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::get(list))
        .route("/:id", routing::get(get))
        .route("/", routing::post(create))
        .route("/:id", routing::put(update))
        .route("/:id", routing::delete(delete))
}

pub async fn list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Notifier>>, ApiError> {
    Ok(Json(state.notifier_service.find_all().await?))
}

pub async fn get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Notifier>, ApiError> {
    Ok(Json(state.notifier_service.find(id).await?))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateNotifier>,
) -> Result<Json<Notifier>, ApiError> {
    let id = state.notifier_service.create(&body).await?;
    Ok(Json(state.notifier_service.find(id).await?))
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(body): Json<CreateNotifier>,
) -> Result<Json<Notifier>, ApiError> {
    let id = state.notifier_service.update(id, &body).await?;
    Ok(Json(state.notifier_service.find(id).await?))
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<(), ApiError> {
    state.notifier_service.delete(id).await?;
    Ok(())
}
