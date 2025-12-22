use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenQuery {
    pub access_token: String,
}

fn extract_token_header(request: &Request) -> Option<String> {
    request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(String::from)
}

fn extract_token_query(request: &Request) -> Option<String> {
    let query_str = request.uri().query()?;
    let query: TokenQuery = serde_qs::from_str(query_str).ok()?;
    Some(query.access_token)
}

fn extract_token(request: &Request) -> Option<String> {
    extract_token_header(request).or_else(|| extract_token_query(request))
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let settings = state.settings_service.get().await;
    if !settings.auth.enabled {
        return Ok(next.run(request).await);
    }

    let token = extract_token(&request).ok_or(StatusCode::UNAUTHORIZED)?;

    state
        .jwt_service
        .decode::<()>(token.as_str())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(request).await)
}
