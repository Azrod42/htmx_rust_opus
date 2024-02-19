use askama_axum::IntoResponse;
use axum::{
    http::{header, Response, StatusCode},
    Json,
};
use bcrypt::{bcrypt, hash};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{query, Row};

use crate::{
    pages::templates::{DashboardBody, Login},
    structs::{
        auth::{LoginPayload, RegisterPayload},
        database::DatabaseConnection,
        entity::user::{User, UserId},
        jwt_token::JwtToken,
        regex::RegexPattern,
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

pub async fn user_register(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_result = sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .map(|row: sqlx::postgres::PgRow| UserId { id: row.get(0) })
        .fetch_one(&mut *conn)
        .await;

    match &user_result {
        Ok(_) => return Err((StatusCode::UNAUTHORIZED, "User already exist".to_string())),
        Err(_) => {}
    };

    if !RegexPattern::Email.is_match(&payload.email) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid email".to_string()));
    }

    if payload.password != payload.password_confirm {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Password didn't match".to_string(),
        ));
    }

    match &payload.password.len() {
        0..=5 => return Err((StatusCode::UNAUTHORIZED, "Password to small".to_string())),
        40.. => return Err((StatusCode::UNAUTHORIZED, "Password to big".to_string())),
        _ => {}
    }

    match &payload.username.len() {
        0..=3 => return Err((StatusCode::UNAUTHORIZED, "Username to small".to_string())),
        20.. => return Err((StatusCode::UNAUTHORIZED, "Username to big".to_string())),
        _ => {}
    }

    let hashed_password = hash(&payload.password, 11).unwrap();

    let result = query!(
        r#"INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING id, username"#,
        payload.username,
        hashed_password,
        payload.email
    )
    .fetch_one(&mut *conn)
    .await;

    match &result {
        Ok(_) => Ok(Login {}),
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid log".to_string())),
    }
}
