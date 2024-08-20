#![forbid(unsafe_code)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]

use crate::route_handler::{command_handler, query_handler};
use crate::state::new_application_state;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

mod config;
mod domain;
mod metadata_extractor;
mod queries;
mod route_handler;
mod services;
mod state;

#[tokio::main]
async fn main() {
    let state = new_application_state().await;

    // Configure the Axum routes and services.
    // For this example a single logical endpoint is used and the HTTP method
    // distinguishes whether the call is a command or a query.
    let router = Router::new()
        .route(
            "/account/:account_id",
            get(query_handler).post(command_handler),
        )
        .with_state(state);

    // Start the Axum server.
    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
