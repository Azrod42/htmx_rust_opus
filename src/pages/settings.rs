use askama::Template;
use axum::Extension;

use crate::structs::entity::user::User;

#[derive(Template)]
#[template(path = "pages/settings.html")]
pub struct SettingsPage {}

pub async fn settings_page() -> SettingsPage {
    SettingsPage {}
}

#[derive(Template)]
#[template(path = "components/settings/profile.html")]
pub struct SettingsProfile {
    pub username: String,
    pub email: String,
    pub lon: f32,
    pub lat: f32,
}

pub async fn settings_profile(Extension(user): Extension<User>) -> SettingsProfile {
    println!("{:#?}", user);
    SettingsProfile {
        username: user.username,
        email: user.email,
        lon: user.lon,
        lat: user.lat,
    }
}
