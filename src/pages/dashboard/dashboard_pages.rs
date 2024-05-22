use axum::Extension;

use crate::{
    pages::settings::weather::get_user_weather,
    structs::{
        self,
        database::DatabaseConnection,
        entity::{user::User, weather::Weather},
    },
};

use super::dashboard_templates::{Dashboard, DashboardHome, DashboardHomeWeather, DashboardTools};

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

pub async fn dashboard(Extension(user): Extension<User>) -> Dashboard {
    Dashboard {
        username: user.username,
    }
}

pub async fn dashboard_home() -> DashboardHome {
    DashboardHome {}
}

pub async fn dashboard_tools() -> DashboardTools {
    DashboardTools {}
}
