use super::templates::{Dashboard, DashboardHome, DashboardTools, ToolsMain};

pub async fn dashboard() -> Dashboard {
    Dashboard {}
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
