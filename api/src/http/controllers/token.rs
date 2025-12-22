use std::sync::Arc;

use axum::{extract::State, routing, Json, Router};

use crate::{
    http::error::ApiError,
    models::token::{CreateToken, TokenPair},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", routing::post(create))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateToken>,
) -> Result<Json<TokenPair>, ApiError> {
    let settings = state.settings_service.get().await;
    if settings.auth.enabled {
        match &settings.auth.credentials {
            None => return Err(ApiError::Forbidden()),
            Some(credentials) => {
                if credentials.username != body.username || credentials.password != body.password {
                    return Err(ApiError::Forbidden());
                }
            }
        }
    }
    let access = state.jwt_service.encode((), 60)?;
    let refresh = state.jwt_service.encode((), 3600)?;
    Ok(Json(TokenPair { access, refresh }))
}
