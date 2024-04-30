use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeFile};

use crate::{
    index,
    pages::{
        auth::{user_login_page, user_register_page},
        components::index_visit,
        dashboard::{dashboard, dashboard_home, dashboard_tools, tools_main},
        settings::settings_page,
    },
};

use super::{
    auth::{logout, user_login, user_register},
    jwt_auth,
};

pub fn services_routes(pool: sqlx::PgPool) -> Router {
    let app = Router::new()
        .route_service("/css/global.css", ServeFile::new("statics/global.css"))
        .route_service("/css/index.css", ServeFile::new("statics/index.css"))
        .route_service(
            "/css/dashboard.css",
            ServeFile::new("statics/dashboard.css"),
        )
        .route_service(
            "/css/components/auth/auth.css",
            ServeFile::new("statics/components/auth/auth.css"),
        )
        .route_service(
            "/css/inputs.css",
            ServeFile::new("statics/components/inputs.css"),
        )
        .route_service(
            "/css/feedback.css",
            ServeFile::new("statics/components/feedback.css"),
        )
        .route_service(
            "/css/components/tools.css",
            ServeFile::new("statics/components/tools.css"),
        )
        .route_service(
            "/icons/github.svg",
            ServeFile::new("statics/images/logos/github.svg"),
        )
        .route_service(
            "/icons/in.svg",
            ServeFile::new("statics/images/logos/in.svg"),
        )
        .route_service(
            "/icons/ig.svg",
            ServeFile::new("statics/images/logos/ig.svg"),
        )
        .route_service(
            "/icons/stack.svg",
            ServeFile::new("statics/images/logos/stack.svg"),
        )
        .route_service("/pp.png", ServeFile::new("statics/images/pp.png"))
        .route_service("/setup.webp", ServeFile::new("statics/images/setup.webp"))
        .route_service("/icons/x.svg", ServeFile::new("statics/images/logos/x.svg"))
        .route_service("/favicon.ico", ServeFile::new("statics/images/favicon.ico"))
        .with_state(pool)
        .layer(CorsLayer::permissive());
    app
}

pub fn auth_routes(pool: sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route("/register", get(user_register_page))
        .route("/login", get(user_login_page))
        .route("/login", post(user_login))
        .route("/register", post(user_register))
        .route("/logout", get(logout))
        .with_state(pool)
        .layer(CorsLayer::permissive());
    app
}
pub fn folio_routes(pool: sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route("/", get(index))
        .route("/visit", get(index_visit))
        .route("/clear-element", get(""))
        .with_state(pool)
        .layer(CorsLayer::permissive());
    app
}

pub fn settings_routes(pool: sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route(
            "/settings",
            get(settings_page).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                jwt_auth::check_user_auth,
            )),
        )
        .with_state(pool)
        .layer(CorsLayer::permissive());
    app
}

pub fn dashboard_routes(pool: sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route(
            "/dashboard",
            get(dashboard).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                jwt_auth::check_user_auth,
            )),
        )
        .route(
            "/dashboard-home",
            get(dashboard_home).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                jwt_auth::check_user_auth,
            )),
        )
        .route(
            "/dashboard-tools",
            get(dashboard_tools).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                jwt_auth::check_user_auth,
            )),
        )
        .with_state(pool)
        .layer(CorsLayer::permissive());
    app
}

pub fn tools_routes(pool: sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route(
            "/main",
            get(tools_main).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                jwt_auth::check_user_auth,
            )),
        )
        .with_state(pool)
        .layer(CorsLayer::permissive());
    app
}
