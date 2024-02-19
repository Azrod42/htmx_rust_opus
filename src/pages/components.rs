use axum::http::StatusCode;

use super::templates::Snackbar;

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
