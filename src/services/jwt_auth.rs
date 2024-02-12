use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::{bson::doc, Client, Collection};
use serde::Serialize;

use crate::{
    pages::templates::Login,
    structs::{entity::user::User, jwt_token::JwtToken},
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth(
    cookie_jar: CookieJar,
    State(client): State<Client>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Login)> {
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
        return Err((StatusCode::OK, Login {}));
    }

    let jwt_secret = std::env::var("JWT_TOKEN").expect("JWT_TOKEN is unset");
    let claims = decode::<JwtToken>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::OK, Login {}))?
    .claims;

    let query = doc! {"email": &claims.sub};
    let coll: Collection<User> = client.database("test").collection::<User>("users");
    let user = coll.find_one(query, None).await;

    match &user {
        Ok(expr) => {
            if !expr.is_some() {
                return Err((StatusCode::OK, Login {}));
            }
        }
        Err(_) => return Err((StatusCode::OK, Login {})),
    }

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
