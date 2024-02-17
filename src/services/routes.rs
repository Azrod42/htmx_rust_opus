use axum::{routing::get, Router};
use tower_http::services::ServeFile;

use crate::{
    index,
    pages::{
        auth::{user_login_page, user_register_page},
        dashboard::dashboard,
    },
};

fn global_routes(app: axum::routing::Router) -> Router {
    app.route("/", get(index))
        .route("/dashboard", get(dashboard))
}

fn services_routes(app: axum::routing::Router) -> Router {
    app.route_service("/css/global.css", ServeFile::new("statics/global.css"))
        .route_service(
            "/css/components/auth/auth.css",
            ServeFile::new("statics/components/auth/auth.css"),
        )
        .route_service(
            "/css/textfield.css",
            ServeFile::new("statics/components/inputs/textfield.css"),
        )
        .route_service(
            "/css/button.css",
            ServeFile::new("statics/components/inputs/button.css"),
        )
        .route_service("/favicon.ico", ServeFile::new("statics/images/favicon.ico"))
}

fn auth_routes(app: axum::routing::Router) -> axum::routing::Router {
    app.route("/register", get(user_register_page))
        .route("/login", get(user_login_page))
}

pub fn init_routes(app: axum::routing::Router) -> axum::routing::Router {
    let app = global_routes(app);
    let app = services_routes(app);
    auth_routes(app)
}
