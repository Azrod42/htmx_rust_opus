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
        dashboard::dashboard,
        dashboard_props::dashboard_props,
    },
};

use super::{
    auth::{user_login, user_register},
    jwt_auth,
};

fn global_routes(app: axum::routing::Router) -> Router {
    app.route("/", get(index))
        .route("/dashboard", get(dashboard))
        .route("/clear-element", get(""))
}

fn services_routes(app: axum::routing::Router, _pool: sqlx::PgPool) -> Router {
    app.route_service("/css/global.css", ServeFile::new("statics/global.css"))
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
}

fn auth_routes(app: axum::routing::Router) -> axum::routing::Router {
    app.route("/register", get(user_register_page))
        .route("/login", get(user_login_page))
}

pub fn init_routes(pool: sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route("/login", post(user_login))
        .route("/register", post(user_register))
        .route("/visit", get(index_visit))
        .route(
            "/dashboard-props",
            get(dashboard_props).route_layer(middleware::from_fn_with_state(
                pool.clone(),
                jwt_auth::check_user_auth,
            )),
        )
        .with_state(pool.clone())
        .layer(CorsLayer::permissive());

    let app = global_routes(app);
    let app = services_routes(app, pool);
    auth_routes(app)
}
