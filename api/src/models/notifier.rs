use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    models::event::Event,
    state::AppState,
    worker::notifiers::discord::{notify_discord, DiscordParams},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NotifierParams {
    Discord(DiscordParams),
}

impl NotifierParams {
    pub async fn send(self, state: Arc<AppState>, event: Arc<Event>) -> Result<()> {
        match self {
            Self::Discord(params) => notify_discord(NotifierContext {
                params,
                state,
                event,
            }),
        }
        .await
    }
}

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct Notifier {
    pub id: i64,
    pub created_at: String,
    pub params: NotifierParams,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct CreateNotifier {
    pub params: NotifierParams,
}

pub struct NotifierContext<T> {
    pub params: T,
    pub event: Arc<Event>,
    pub state: Arc<AppState>,
}
