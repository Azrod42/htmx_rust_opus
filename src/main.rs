mod pages;
mod services;
mod structs;

use std::env;

use askama_axum::IntoResponse;
use axum::http::StatusCode;

use crate::{
    services::routes::{
        auth_routes, dashboard_routes, folio_routes, services_routes, settings_routes, tools_routes,
    },
    structs::database::init_database,
};

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::var("JWT_TOKEN").expect("JWT_TOKEN not set");
    env::var("DATABASE_URL").expect("DATABASE_URL not set");
    env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not set");

    let pool_database = init_database().await;

    let nested_routes = auth_routes(&pool_database)
        .merge(services_routes(&pool_database))
        .merge(settings_routes(&pool_database))
        .merge(folio_routes(&pool_database))
        .merge(dashboard_routes(&pool_database))
        .nest("/tools", tools_routes(pool_database));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4270").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, nested_routes).await.unwrap();
    Ok(())
}
