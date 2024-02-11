use askama_axum::IntoResponse;
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{bson::doc, Client, Collection};

use crate::{
    pages::templates::DashboardBody,
    structs::{auth::LoginPayload, entity::user::User, jwt_token::JwtToken},
};
use axum_extra::extract::cookie::{Cookie, SameSite};

pub async fn user_login(
    State(client): State<Client>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query = doc! {"email": &payload.email};
    let coll: Collection<User> = client.database("test").collection::<User>("users");
    let user = coll.find_one(query, None).await;

    match &user {
        Ok(expr) => {
            if !expr.is_some() {
                return Err((StatusCode::UNAUTHORIZED, "Invalid log".to_string()));
            }
        }
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid log".to_string())),
    }

    let user = user.unwrap().unwrap();
    let uno = bcrypt::verify(payload.password, &user.password);

    match uno {
        Ok(value) => {
            if value {
                println!("{} login", &payload.email)
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
