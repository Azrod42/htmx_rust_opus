use askama::Template;

use crate::structs::entity::weather::Weather;

#[derive(Template)]
#[template(path = "pages/dashboard.html")]
pub struct Dashboard {
    pub username: String,
}

#[derive(Template)]
#[template(path = "components/dashboard/home/home.html")]
pub struct DashboardHome {}

#[derive(Template)]
#[template(path = "components/dashboard/home/home-weather.html")]
pub struct DashboardHomeWeather {
    pub(crate) weather: Weather,
}

#[derive(Template)]
#[template(path = "components/dashboard/tools.html")]
pub struct DashboardTools {}

#[derive(Template)]
#[template(path = "components/tools/main.html")]
pub struct ToolsMain {}

#[derive(Template)]
#[template(path = "components/tools/chat.html")]
pub struct ToolsChat {}

#[derive(Template)]
#[template(path = "components/wasm/main.html")]
pub struct DashboardWasm {}
