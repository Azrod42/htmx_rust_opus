use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "components/auth/login.html")]
pub struct Login {}

#[derive(Template)]
#[template(path = "components/auth/register.html")]
pub struct Register {}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard {}

#[derive(Template)]
#[template(path = "dashboard_props.html")]
pub struct DashboardBody {}

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
