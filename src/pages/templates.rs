use askama::Template;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "components/auth/login.html")]
pub struct Login {}

#[derive(Template)]
#[template(path = "components/auth/loginSuccess.html")]
pub struct LoginSuccess {}

#[derive(Template)]
#[template(path = "pages/auth.html")]
pub struct AuthPage {}

#[derive(Template)]
#[template(path = "components/auth/register.html")]
pub struct Register {}

#[derive(Template)]
#[template(path = "pages/dashboard.html")]
pub struct Dashboard {}

#[derive(Template)]
#[template(path = "components/dashboard/home.html")]
pub struct DashboardHome {}

#[derive(Template)]
#[template(path = "components/dashboard/tools.html")]
pub struct DashboardTools {}

#[derive(Template)]
#[template(path = "components/tools/main.html")]
pub struct ToolsMain {}

#[derive(Template)]
#[template(path = "components/snackbar.html")]
pub struct Snackbar {
    pub status: String,
    pub message: String,
    pub color: String,
}

#[derive(Template)]
#[template(path = "components/index/visit.html")]
pub struct IndexVisit {
    pub number_visit: i32,
}
