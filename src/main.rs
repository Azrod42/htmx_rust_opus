mod pages;
mod services;
mod structs;

use std::env;

use askama_axum::IntoResponse;
use axum::http::StatusCode;
use pages::templates::Index;

use crate::{
    services::routes::{
        auth_routes, dashboard_routes, folio_routes, services_routes, tools_routes,
    },
    structs::database::init_database,
};

async fn index() -> Index {
    Index {}
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::var("JWT_TOKEN").expect("JWT_TOKEN not set");
    env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool_database = init_database().await;
    let services_routes = services_routes(pool_database.clone());
    let auth_routes = auth_routes(pool_database.clone());
    let folio_routes = folio_routes(pool_database.clone());
    let dashboard_routes = dashboard_routes(pool_database.clone());
    let tools_routes = tools_routes(pool_database);
    let nest = auth_routes
        .nest("/tools", tools_routes)
        .merge(services_routes)
        .merge(folio_routes)
        .merge(dashboard_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4270").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, nest).await.unwrap();
    Ok(())
}
