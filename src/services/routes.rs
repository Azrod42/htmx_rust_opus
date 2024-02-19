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
        dashboard::dashboard,
        dashboard_props::dashboard_props,
    },
};

use super::{
    auth::{user_login, user_register},
    jwt_auth::auth,
};

fn global_routes(app: axum::routing::Router) -> Router {
    app.route("/", get(index))
        .route("/dashboard", get(dashboard))
        .route("/clear-element", get(""))
}

fn services_routes(app: axum::routing::Router) -> Router {
    app.route_service("/css/global.css", ServeFile::new("statics/global.css"))
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
        .route_service("/favicon.ico", ServeFile::new("statics/images/favicon.ico"))
}

fn auth_routes(app: axum::routing::Router) -> axum::routing::Router {
    app.route("/register", get(user_register_page))
        .route("/login", get(user_login_page))
}

pub fn init_routes(pool: sqlx::PgPool) -> axum::routing::Router {
    let app = Router::new()
        .route("/login", post(user_login))
        // .route("/test", get(snackbar))
        .route("/register", post(user_register))
        .route(
            "/dashboard-props",
            get(dashboard_props).route_layer(middleware::from_fn_with_state(pool.clone(), auth)),
        )
        .with_state(pool)
        .layer(CorsLayer::permissive());

    let app = global_routes(app);
    let app = services_routes(app);
    auth_routes(app)
}
