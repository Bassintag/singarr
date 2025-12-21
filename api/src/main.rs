use axum::Router;
use clap::Parser;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::{
    args::AppArgs,
    http::controllers::{album, artist, job, lyrics, search, settings, socket, track},
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
        .nest("/albums", album::routes())
        .nest("/artists", artist::routes())
        .nest("/jobs", job::routes())
        .nest("/lyrics", lyrics::routes())
        .nest("/settings", settings::routes())
        .nest("/search", search::routes())
        .nest("/socket", socket::routes())
        .nest("/tracks", track::routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
