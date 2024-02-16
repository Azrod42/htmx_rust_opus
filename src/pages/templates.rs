// #[derive(Debug, Template)]
// #[template(path = "timer.html")]
// pub struct Timer {
//     pub oob: bool,
//     pub msg: String,
// }

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
