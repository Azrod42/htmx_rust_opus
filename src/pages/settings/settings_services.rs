use axum::Extension;
use sqlx::Row;

use crate::structs::{database::DatabaseConnection, entity::user::User};

use super::settings_templates::{SettingsPage, SettingsProfile};

pub async fn settings_page() -> SettingsPage {
    SettingsPage {}
}

pub async fn settings_profile(
    Extension(user): Extension<User>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> SettingsProfile {
    let user_result =
        sqlx::query("SELECT lon, lat, open_weather_api_key FROM users WHERE email = $1")
            .bind(&user.email)
            .map(|row: sqlx::postgres::PgRow| User {
                lon: row.get(0),
                lat: row.get(1),
                open_weather_api_key: row.get(2),
                ..Default::default()
            })
            .fetch_one(&mut *conn)
            .await;

    let user_data = user_result.unwrap_or(User {
        ..Default::default()
    });

    SettingsProfile {
        username: user.username,
        email: user.email,
        lon: user_data.lon,
        lat: user_data.lat,
        open_weather_api_key: user_data.open_weather_api_key,
    }
}
