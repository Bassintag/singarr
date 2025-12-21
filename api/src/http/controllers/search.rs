use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing, Json, Router,
};

use crate::{
    http::error::ApiError,
    models::search::{CreateSearchQuery, Search},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    return Router::new().route("/", routing::get(create));
}

async fn create(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CreateSearchQuery>,
) -> Result<Json<Vec<Search>>, ApiError> {
    Ok(Json(state.search_service.create(&query.q).await?))
}
