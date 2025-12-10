use std::sync::Arc;

use axum::Router;
use clap::Parser;
use dotenv::dotenv;
use tokio::net::TcpListener;

use crate::{
    args::AppArgs,
    http::controllers::{album, artist, lyrics, settings, track},
    state::AppState,
};

pub mod args;
pub mod state;

pub mod http;
pub mod models;
pub mod services;
pub mod worker;

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let args = AppArgs::parse();
    let state = AppState::from_args(args.clone()).await.unwrap();

    let app = Router::new()
        .nest("/albums", album::routes())
        .nest("/artists", artist::routes())
        .nest("/lyrics", lyrics::routes())
        .nest("/settings", settings::routes())
        .nest("/tracks", track::routes())
        .with_state(Arc::new(state));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
