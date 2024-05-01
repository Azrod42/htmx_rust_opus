use askama::Template;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct Index {}

pub async fn index_page() -> Index {
    Index {}
}
