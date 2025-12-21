use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::state::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(upgrade))
}

pub async fn upgrade(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| {
        if let Err(e) = handle_socket(state, socket).await {
            println!("Error whild handling socket: {}", e);
        }
    })
}

async fn handle_socket(state: Arc<AppState>, mut ws: WebSocket) -> Result<()> {
    let mut reciever = state.event_service.subscribe();
    while let Ok(e) = reciever.recv().await {
        if ws.send(serde_json::to_string(&e)?.into()).await.is_err() {
            break;
        }
    }
    Ok(())
}
