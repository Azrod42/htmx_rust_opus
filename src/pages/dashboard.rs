use askama::Template;
use axum::Extension;

use crate::structs::entity::user::User;

#[derive(Template)]
#[template(path = "pages/dashboard.html")]
pub struct Dashboard {
    pub username: String,
}

#[derive(Template)]
#[template(path = "components/dashboard/home.html")]
pub struct DashboardHome {}

#[derive(Template)]
#[template(path = "components/dashboard/tools.html")]
pub struct DashboardTools {}

#[derive(Template)]
#[template(path = "components/tools/main.html")]
pub struct ToolsMain {}

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

pub async fn tools_main() -> ToolsMain {
    ToolsMain {}
}
