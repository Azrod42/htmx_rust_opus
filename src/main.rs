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
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

use crate::{
    pages::dashboard_props::dashboard_props,
    services::{auth::user_login, jwt_auth::auth, routes::init_routes},
};

async fn index() -> Index {
    Index {}
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/login", post(user_login))
        .route("/register", post(user_login))
        .route(
            "/dashboard-props",
            get(dashboard_props).route_layer(middleware::from_fn_with_state(pool.clone(), auth)),
        )
        .with_state(pool)
        .layer(CorsLayer::permissive());
    let app = init_routes(app);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
