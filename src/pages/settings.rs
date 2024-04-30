use askama::Template;

#[derive(Template)]
#[template(path = "pages/settings.html")]
pub struct SettingsPage {}

pub async fn settings_page() -> SettingsPage {
    SettingsPage {}
}
