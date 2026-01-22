use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};

use crate::{
    http::error::ApiError,
    models::provider::{ProviderResult, ResultsQuery},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/results", get(list_results))
}

async fn list_results(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ResultsQuery>,
) -> Result<Json<Vec<ProviderResult>>, ApiError> {
    let track = state.track_service.find(query.track_id).await?;
    Ok(Json(state.provider_service.get_results(&track).await?))
}
