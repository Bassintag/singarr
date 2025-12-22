use std::sync::Arc;

use axum::{extract::State, routing, Json, Router};

use crate::{http::error::ApiError, models::settings::Settings, state::AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::get(get))
        .route("/", routing::put(set))
}

async fn get(State(state): State<Arc<AppState>>) -> Json<Settings> {
    Json(state.settings_service.get().await)
}

async fn set(
    State(state): State<Arc<AppState>>,
    Json(data): Json<Settings>,
) -> Result<Json<Settings>, ApiError> {
    state.settings_service.set(data.clone()).await?;
    Ok(Json(data))
}
