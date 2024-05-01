use askama::Template;
use askama_axum::IntoResponse;
use axum::http::{HeaderMap, StatusCode};
use sqlx::{query, Row};

use crate::structs::{database::DatabaseConnection, entity::visit::VisitId};

#[derive(Template)]
#[template(path = "components/snackbar.html")]
pub struct Snackbar {
    pub status: String,
    pub message: String,
    pub color: String,
}

pub fn return_snackbar(
    status: String,
    message: String,
    color: Option<String>,
) -> (StatusCode, Snackbar) {
    (
        StatusCode::OK,
        Snackbar {
            status,
            message,
            color: color.unwrap_or(String::from("")),
        },
    )
}

#[derive(Template)]
#[template(path = "components/index/visit.html")]
pub struct IndexVisit {
    pub number_visit: i32,
}

pub async fn index_visit(
    headers: HeaderMap,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, (StatusCode, IndexVisit)> {
    let user_agent = headers.get("user-agent");
    let user_agent: Option<&str> = match &user_agent {
        Some(agent) => Some(agent.to_str().unwrap_or("none")),
        None => None,
    };

    let result = query(
        r#"INSERT INTO visit (date, user_agent) VALUES (CURRENT_TIMESTAMP, $1) RETURNING id"#,
    )
    .bind(user_agent)
    .map(|row: sqlx::postgres::PgRow| VisitId { id: row.get(0) })
    .fetch_one(&mut *conn)
    .await;

    let visit: VisitId = match &result {
        Ok(row) => row.clone(),
        Err(_) => return Err((StatusCode::BAD_REQUEST, IndexVisit { number_visit: 0 })),
    };
    Ok((
        StatusCode::OK,
        IndexVisit {
            number_visit: visit.id,
        },
    ))
}

#[derive(Template)]
#[template(path = "components/layout/top-bar-menu.html")]
pub struct TopBarMenu {}

pub async fn top_bar_menu() -> TopBarMenu {
    TopBarMenu {}
}
