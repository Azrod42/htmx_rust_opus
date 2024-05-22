use askama::Template;

#[derive(Template)]
#[template(path = "pages/settings.html")]
pub struct SettingsPage {}

#[derive(Template)]
#[template(path = "components/settings/profile.html")]
pub struct SettingsProfile {
    pub username: String,
    pub email: String,
    pub lon: f32,
    pub lat: f32,
    pub open_weather_api_key: String,
}
