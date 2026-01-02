use std::sync::Arc;

use axum::{middleware, Router};
use clap::Parser;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::{
    args::AppArgs,
    http::{
        controllers::{
            album, artist, job, lyrics, notifier, search, settings, socket, stats, status, tasks,
            token, track,
        },
        middlewares::auth::auth_middleware,
    },
    state::AppState,
};

pub mod args;
pub mod state;

pub mod http;
pub mod models;
pub mod services;
pub mod utils;
pub mod worker;

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let args = AppArgs::parse();
    let state = AppState::from_args(args.clone()).await.unwrap();

    let app = Router::new()
        .nest("/", private_routes(state.clone()))
        .nest("/", public_routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn private_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .nest("/albums", album::routes())
        .nest("/artists", artist::routes())
        .nest("/jobs", job::routes())
        .nest("/lyrics", lyrics::routes())
        .nest("/notifiers", notifier::routes())
        .nest("/settings", settings::routes())
        .nest("/search", search::routes())
        .nest("/socket", socket::routes())
        .nest("/stats", stats::routes())
        .nest("/tasks", tasks::routes())
        .nest("/tracks", track::routes())
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/tokens", token::routes())
        .nest("/status", status::routes())
}
