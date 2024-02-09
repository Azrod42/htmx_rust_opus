mod pages;
mod structs;

use std::env;

use askama_axum::IntoResponse;
use axum::{http::StatusCode, routing::get, Router};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use pages::templates::Index;
use tower_http::services::ServeFile;

use crate::{pages::login::user_login_page, structs::common::DatabaseConfig};

async fn index() -> Index {
    Index {}
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() {
    env::set_var("MONGO_MAX_POOL_SIZE", "4");

    let database_config = DatabaseConfig::new();
    let mut client_options = ClientOptions::parse(database_config.uri).await.unwrap();
    client_options.connect_timeout = database_config.connection_timeout;
    client_options.max_pool_size = database_config.max_pool_size;
    client_options.min_pool_size = database_config.min_pool_size;
    client_options.compressors = database_config.compressors;
    let client = Client::with_options(client_options).unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/dashboard", get(user_login_page))
        .route_service("/css/global.css", ServeFile::new("statics/global.css"));

    let app = app.fallback(handler_404).with_state(client);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
