mod pages;
mod services;
mod structs;

use std::env;

use askama_axum::IntoResponse;
use axum::{
    http::StatusCode,
    middleware,
    routing::{get, post},
    Router,
};
use pages::templates::Index;
use tower_http::services::ServeFile;

use crate::{
    pages::{dashboard::dashboard, dashboard_props::dashboard_props},
    services::{auth::user_login, database, jwt_auth::auth},
};

async fn index() -> Index {
    Index {}
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() {
    env::set_var("MONGO_MAX_POOL_SIZE", "4");

    let client = database::init_database().await;

    let app = Router::new()
        .route("/", get(index))
        .route("/dashboard", get(dashboard))
        .route(
            "/dashboard-props",
            get(dashboard_props).route_layer(middleware::from_fn_with_state(client.clone(), auth)),
        )
        .route("/login", post(user_login))
        .route_service("/css/global.css", ServeFile::new("statics/global.css"))
        .route_service(
            "/css/components/auth/login.css",
            ServeFile::new("statics/components/auth/login.css"),
        )
        .route_service(
            "/css/textfield.css",
            ServeFile::new("statics/components/inputs/textfield.css"),
        )
        .route_service(
            "/css/button.css",
            ServeFile::new("statics/components/inputs/button.css"),
        )
        .route_service("/favicon.ico", ServeFile::new("statics/images/favicon.ico"));

    let app = app.fallback(handler_404).with_state(client);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
