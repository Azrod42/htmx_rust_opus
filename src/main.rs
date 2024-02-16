mod pages;
mod services;
mod structs;

use std::env;

use askama_axum::IntoResponse;
use axum::http::StatusCode;
use pages::templates::Index;

use crate::services::{database, routes::init_routes};

async fn index() -> Index {
    Index {}
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("MONGO_MAX_POOL_SIZE", "4");

    let client = database::init_database().await;
    let app = init_routes(client);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
