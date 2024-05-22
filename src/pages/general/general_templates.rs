use askama::Template;

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

#[derive(Template)]
#[template(path = "components/layout/top-bar-menu.html")]
pub struct TopBarMenu {}

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct Index {}
