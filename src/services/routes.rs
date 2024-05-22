use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeFile};

use crate::pages::{
    auth::{
        auth_jwt::check_user_auth,
        auth_pages::{user_login_page, user_register_page},
        auth_services::{logout, user_login, user_register},
    },
    dashboard::{
        dashboard_pages::{dashboard, dashboard_home, dashboard_home_weather, dashboard_tools},
        dashboard_templates::tools_main,
    },
    general::general_services::{index_page, index_visit, top_bar_menu},
    settings::{
        settings_pages::settings_update_profile,
        settings_services::{settings_page, settings_profile},
    },
};

pub fn auth_routes(pool: &sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route("/register", get(user_register_page))
        .route("/login", get(user_login_page))
        .route("/login", post(user_login))
        .route("/register", post(user_register))
        .route("/logout", get(logout))
        .with_state(pool.clone())
        .layer(CorsLayer::permissive());
    app
}
pub fn folio_routes(pool: &sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route("/", get(index_page))
        .route("/visit", get(index_visit))
        .route("/clear-element", get(""))
        .with_state(pool.clone())
        .layer(CorsLayer::permissive());
    app
}

pub fn settings_routes(pool: &sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route(
            "/settings",
            get(settings_page).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                check_user_auth,
            )),
        )
        .route(
            "/settings/profile",
            get(settings_profile).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                check_user_auth,
            )),
        )
        .route(
            "/settings/profile",
            post(settings_update_profile).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                check_user_auth,
            )),
        )
        .with_state(pool.clone())
        .layer(CorsLayer::permissive());
    app
}

pub fn dashboard_routes(pool: &sqlx::PgPool) -> axum::routing::Router {
    let middleware = middleware::from_fn_with_state(pool.clone(), check_user_auth);
    let app = Router::new()
        .route("/dashboard", get(dashboard).route_layer(middleware.clone()))
        .route(
            "/dashboard-home",
            get(dashboard_home).route_layer(middleware.clone()),
        )
        .route(
            "/dashboard-home/weather",
            get(dashboard_home_weather).route_layer(middleware.clone()),
        )
        .route(
            "/top-bar-menu",
            get(top_bar_menu).route_layer(middleware.clone()),
        )
        .route(
            "/dashboard-tools",
            get(dashboard_tools).route_layer(middleware),
        )
        .with_state(pool.clone())
        .layer(CorsLayer::permissive());
    app
}

pub fn tools_routes(pool: sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route(
            "/main",
            get(tools_main).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                check_user_auth,
            )),
        )
        .with_state(pool.clone())
        .layer(CorsLayer::permissive());
    app
}

pub fn services_routes(pool: &sqlx::PgPool) -> Router {
    let app = Router::new()
        .route_service("/css/global.css", ServeFile::new("statics/css/global.css"))
        .route_service("/css/index.css", ServeFile::new("statics/css/index.css"))
        .route_service(
            "/css/dashboard.css",
            ServeFile::new("statics/css/dashboard.css"),
        )
        .route_service(
            "/css/settings.css",
            ServeFile::new("statics/css/settings.css"),
        )
        .route_service(
            "/css/components/auth/auth.css",
            ServeFile::new("statics/css/components/auth/auth.css"),
        )
        .route_service(
            "/css/inputs.css",
            ServeFile::new("statics/css/components/inputs.css"),
        )
        .route_service(
            "/css/feedback.css",
            ServeFile::new("statics/css/components/feedback.css"),
        )
        .route_service(
            "/css/components/tools.css",
            ServeFile::new("statics/css/components/tools.css"),
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
        .route_service(
            "/statics/images/loader.svg",
            ServeFile::new("statics/images/loader.svg"),
        )
        .route_service("/pp.png", ServeFile::new("statics/images/pp.png"))
        .route_service("/setup.webp", ServeFile::new("statics/images/setup.webp"))
        .route_service("/icons/x.svg", ServeFile::new("statics/images/logos/x.svg"))
        .route_service("/favicon.ico", ServeFile::new("statics/images/favicon.ico"))
        .with_state(pool.clone())
        .layer(CorsLayer::permissive());
    app
}
