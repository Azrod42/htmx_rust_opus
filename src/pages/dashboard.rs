use askama::Template;
use axum::Extension;

use crate::{
    services::weather::get_user_weather,
    structs::{
        self,
        database::DatabaseConnection,
        entity::{user::User, weather::Weather},
    },
};

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
pub struct DashboardHomeWeather {
    weather: Weather,
}

pub async fn dashboard_home_weather(
    Extension(user): Extension<User>,
    DatabaseConnection(conn): DatabaseConnection,
) -> DashboardHomeWeather {
    let weather = get_user_weather(user, structs::database::DatabaseConnection(conn)).await;

    let weather = match weather {
        Some(data) => data,
        None => {
            return DashboardHomeWeather {
                weather: Weather {
                    ..Default::default()
                },
            }
        }
    };

    DashboardHomeWeather { weather }
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
