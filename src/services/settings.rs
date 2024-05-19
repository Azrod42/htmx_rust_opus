use askama_axum::IntoResponse;
use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::query;

use crate::{
    pages::components::{return_snackbar, Snackbar},
    structs::{database::DatabaseConnection, entity::user::User, enums::SnackbardColor},
};

fn stringdesf32<'de, D>(des: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(des).and_then(|fv| Ok(fv.parse::<f32>().unwrap_or(0.0)))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsProfilePayload {
    pub username: String,
    #[serde(deserialize_with = "stringdesf32")]
    pub lon: f32,
    #[serde(deserialize_with = "stringdesf32")]
    pub lat: f32,
    pub open_weather_api_key: String,
}

pub async fn settings_update_profile(
    Extension(user): Extension<User>,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<SettingsProfilePayload>,
) -> Result<impl IntoResponse, (StatusCode, Snackbar)> {
    match &payload.username.len() {
        0..=2 => {
            return Err(return_snackbar(
                String::from("Error:"),
                String::from("Username is to small"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
        20.. => {
            return Err(return_snackbar(
                String::from("Error:"),
                String::from("Username is to big"),
                Some(SnackbardColor::Error.get_status()),
            ))
        }
        _ => {}
    }

    let result = query!(
        r#"UPDATE users SET username = $1, lon = $2, lat = $3, open_weather_api_key = $4 WHERE email = $5;"#,
        payload.username,
        payload.lon,
        payload.lat,
        payload.open_weather_api_key,
        user.email,
    )
    .execute(&mut *conn)
    .await;

    match &result {
        Ok(_) => Ok(return_snackbar(
            String::from("Sucess:"),
            String::from("Profile save"),
            Some(SnackbardColor::Success.get_status()),
        )),
        Err(_) => Err(return_snackbar(
            String::from("Error:"),
            String::from("Unexpected error append"),
            Some(SnackbardColor::Error.get_status()),
        )),
    }
}
