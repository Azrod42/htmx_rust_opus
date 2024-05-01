use askama_axum::IntoResponse;
use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::query;

use crate::{
    pages::components::{return_snackbar, Snackbar},
    structs::{database::DatabaseConnection, entity::user::User, enums::SnackbardColor},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsProfilePayload {
    pub username: String,
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
        r#"UPDATE users SET username = $1 WHERE email = $2;"#,
        payload.username,
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
