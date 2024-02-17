use askama_axum::IntoResponse;
use axum::{
    http::{header, Response, StatusCode},
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::Row;

use crate::{
    pages::templates::DashboardBody,
    structs::{
        auth::LoginPayload, database::DatabaseConnection, entity::user::User, jwt_token::JwtToken,
    },
};
use axum_extra::extract::cookie::{Cookie, SameSite};

pub async fn user_login(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_result = sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .map(|row: sqlx::postgres::PgRow| User {
            id: row.get(0),
            username: row.get(1),
            email: row.get(2),
            password: row.get(3),
        })
        .fetch_one(&mut *conn)
        .await;

    let user: User = match &user_result {
        Ok(user) => user.clone(),
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid log".to_string())),
    };

    let uno = bcrypt::verify(payload.password, &user.password);

    match uno {
        Ok(value) => {
            if value {
                println!("LOG: user-login: {}", user.email)
            } else {
                return Err((StatusCode::UNAUTHORIZED, "Invalid log".to_string()));
            }
        }
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid log".to_string())),
    }

    let jwt_secret = std::env::var("JWT_TOKEN").expect("JWT_TOKEN is unset");
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(120)).timestamp() as usize;
    let claims: JwtToken = JwtToken {
        sub: user.email,
        iat,
        exp,
    };

    let jwt_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("authentication", jwt_token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(4))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(DashboardBody {}.to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
