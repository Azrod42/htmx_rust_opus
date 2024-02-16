use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeFile;

use crate::{
    index,
    pages::{
        auth::{user_login_page, user_register_page},
        dashboard::dashboard,
        dashboard_props::dashboard_props,
    },
};

use super::{auth::user_login, jwt_auth::auth};

fn pages_routes(db_client: mongodb::Client, app: axum::routing::Router) -> Router {
    app.route("/", get(index))
        .route("/dashboard", get(dashboard))
        .route(
            "/dashboard-props",
            get(dashboard_props)
                .route_layer(middleware::from_fn_with_state(db_client.clone(), auth)),
        )
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

pub fn init_routes(db_client: mongodb::Client) -> axum::routing::Router {
    let app = Router::new()
        .route("/login", post(user_login))
        .with_state(db_client.clone());
    let app = pages_routes(db_client, app);
    let app = services_routes(app);
    auth_routes(app)
}
