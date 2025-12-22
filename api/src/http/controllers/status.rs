use std::sync::Arc;

use axum::{extract::State, routing, Json, Router};

use crate::{models::status::Status, state::AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", routing::get(get))
}

pub async fn get(State(state): State<Arc<AppState>>) -> Json<Status> {
    let settings = state.settings_service.get().await;
    Json(Status {
        auth: settings.auth.enabled,
    })
}
