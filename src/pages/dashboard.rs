use askama::Template;
use axum::Extension;

use crate::structs::entity::user::User;

#[derive(Template)]
#[template(path = "pages/dashboard.html")]
pub struct Dashboard {
    pub username: String,
}

pub async fn dashboard(Extension(user): Extension<User>) -> Dashboard {
    Dashboard {
        username: user.username,
    }
}

#[derive(Template)]
#[template(path = "components/dashboard/home/home.html")]
pub struct DashboardHome {}

pub async fn dashboard_home() -> DashboardHome {
    DashboardHome {}
}

#[derive(Template)]
#[template(path = "components/dashboard/home/home-weather.html")]
pub struct DashboardHomeWeather {}

pub async fn dashboard_home_weather() -> DashboardHomeWeather {
    DashboardHomeWeather {}
}

#[derive(Template)]
#[template(path = "components/dashboard/tools.html")]
pub struct DashboardTools {}

pub async fn dashboard_tools() -> DashboardTools {
    DashboardTools {}
}

#[derive(Template)]
#[template(path = "components/tools/main.html")]
pub struct ToolsMain {}

pub async fn tools_main() -> ToolsMain {
    ToolsMain {}
}
