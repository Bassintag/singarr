use std::sync::Arc;

use axum::{extract::State, routing, Json, Router};

use crate::{
    http::error::ApiError,
    models::token::{CreateToken, TokenClaims, TokenPair},
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
        match body {
            CreateToken::Login { username, password } => match &settings.auth.credentials {
                None => return Err(ApiError::Forbidden()),
                Some(credentials) => {
                    if credentials.username != username || credentials.password != password {
                        return Err(ApiError::Forbidden());
                    }
                }
            },
            CreateToken::Refresh { refresh_token } => {
                if let Ok(payload) = state.jwt_service.decode::<TokenClaims>(&refresh_token) {
                    if payload.claims.typ != "refresh" {
                        return Err(ApiError::Forbidden());
                    }
                } else {
                    return Err(ApiError::Forbidden());
                }
            }
        }
    }
    let access = state.jwt_service.encode(
        TokenClaims {
            typ: "access".into(),
        },
        60,
    )?;
    let refresh = state.jwt_service.encode(
        TokenClaims {
            typ: "refresh".into(),
        },
        3600,
    )?;
    Ok(Json(TokenPair { access, refresh }))
}
