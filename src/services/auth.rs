use askama_axum::IntoResponse;
use axum::{
    http::{header, Response, StatusCode},
    Json,
};
use bcrypt::hash;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{query, Row};

use crate::{
    pages::{
        auth::{Login, LoginSuccess},
        components::{return_snackbar, Snackbar},
    },
    structs::{
        auth::{LoginPayload, RegisterPayload},
        database::DatabaseConnection,
        entity::user::{User, UserId},
        enums::SnackbardColor,
        jwt_token::JwtToken,
        regex::RegexPattern,
    },
};
use axum_extra::extract::cookie::{Cookie, SameSite};

pub async fn user_login(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, Snackbar)> {
    let error_status = String::from("Error: ");
    let user_result = sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email.to_lowercase())
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
        Err(_) => {
            return Err(return_snackbar(
                error_status,
                String::from("Invalid credential"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
    };

    let uno = bcrypt::verify(payload.password, &user.password);

    match uno {
        Ok(value) => {
            if value {
                println!("LOG: user-login: {}", user.email)
            } else {
                return Err(return_snackbar(
                    error_status,
                    String::from("Invalid credential"),
                    Some(SnackbardColor::Error.get_status()),
                ));
            }
        }
        Err(_) => {
            return Err(return_snackbar(
                error_status,
                String::from("Invalid credential"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
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

    let mut response = Response::new(LoginSuccess {}.to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn user_register(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, Snackbar)> {
    let error_status = String::from("Error: ");
    let user_result = sqlx::query("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email.to_lowercase())
        .map(|row: sqlx::postgres::PgRow| UserId { id: row.get(0) })
        .fetch_one(&mut *conn)
        .await;

    match &user_result {
        Ok(_) => {
            return Err(return_snackbar(
                error_status,
                String::from("User already exist"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
        Err(_) => {}
    };

    if !RegexPattern::Email.is_match(&payload.email.to_lowercase()) {
        return Err(return_snackbar(
            error_status,
            String::from("Invalid email"),
            Some(SnackbardColor::Error.get_status()),
        ));
    }

    if payload.password != payload.password_confirm {
        return Err(return_snackbar(
            error_status,
            String::from("Password didn't match"),
            Some(SnackbardColor::Error.get_status()),
        ));
    }

    match &payload.password.len() {
        0..=5 => {
            return Err(return_snackbar(
                error_status,
                String::from("Password is to small"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
        40.. => {
            return Err(return_snackbar(
                error_status,
                String::from("Password is to big"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
        _ => {}
    }

    match &payload.username.len() {
        0..=3 => {
            return Err(return_snackbar(
                error_status,
                String::from("Username is to small"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
        20.. => {
            return Err(return_snackbar(
                error_status,
                String::from("Username is to big"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
        _ => {}
    }

    let hashed_password = hash(&payload.password, 11).unwrap();

    let result = query!(
        r#"INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING id, username"#,
        payload.username,
        hashed_password,
        payload.email.to_lowercase()
    )
    .fetch_one(&mut *conn)
    .await;

    match &result {
        Ok(_) => Ok(Login {}),
        Err(_) => Err(return_snackbar(
            error_status,
            String::from("Unexpected error append"),
            Some(SnackbardColor::Error.get_status()),
        )),
    }
}

pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Login)> {
    let cookie = Cookie::build("authentication ")
        .path("/")
        .max_age(time::Duration::hours(4))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(Login {}.to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
