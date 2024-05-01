use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use sqlx::Row;

use crate::{
    pages::auth::AuthPage,
    structs::{database::DatabaseConnection, entity::user::User, jwt_token::JwtToken},
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn check_user_auth(
    cookie_jar: CookieJar,
    DatabaseConnection(mut conn): DatabaseConnection,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, AuthPage)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::COOKIE)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("authentication") {
                        Some(auth_value[15..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.unwrap_or("NA".to_string());
    if token == "NA" {
        return Err((StatusCode::OK, AuthPage {}));
    }

    let jwt_secret = std::env::var("JWT_TOKEN").expect("JWT_TOKEN is unset");
    let claims = decode::<JwtToken>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::OK, AuthPage {}))?
    .claims;

    let user =
        sqlx::query("SELECT id, username, email, password, lon, lat FROM users WHERE email = $1")
            .bind(&claims.sub)
            .map(|row: sqlx::postgres::PgRow| User {
                id: row.get(0),
                username: row.get(1),
                email: row.get(2),
                password: row.get(3),
                lon: row.get(4),
                lat: row.get(5),
            })
            .fetch_one(&mut *conn)
            .await;

    match &user {
        Ok(_) => {}
        Err(_) => return Err((StatusCode::UNAUTHORIZED, AuthPage {})),
    }
    let mut user = user.unwrap_or(User {
        ..Default::default()
    });
    user.password = String::from("");

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
