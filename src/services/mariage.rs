use askama_axum::IntoResponse;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::query;
use sqlx::Row;

use crate::structs::database::DatabaseConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMariage {
  pub number_of_guests: String,
  pub exigences_alimentaires: String,
  pub name: String,
  pub phone: String,
  pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseId{
    pub id: i32,
}

pub async fn mariage_response(
    DatabaseConnection(mut _conn): DatabaseConnection,
    Json(payload): Json<ResponseMariage>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = query(
        r#"INSERT INTO mariage_response (number_of_guests, exigences_alimentaires, name, phone, email) VALUES ($1, $2, $3, $4, $5) RETURNING id"#,
    )
    .bind(payload.number_of_guests)
    .bind(payload.exigences_alimentaires)
    .bind(payload.name)
    .bind(payload.phone)
    .bind(payload.email)
    .map(|row: sqlx::postgres::PgRow| ResponseId { id: row.get(0) })
    .fetch_one(&mut *_conn)
    .await;

    let visit: ResponseId = match &result {
        Ok(row) => row.clone(),
        Err(_) => return Err((StatusCode::BAD_REQUEST, (-1).to_string()))
    };
    Ok((
        StatusCode::OK,
        visit.id.to_string()
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMusic {
  pub sugest_by: String,
  pub musics: String,
}

pub async fn mariage_music(
    DatabaseConnection(mut _conn): DatabaseConnection,
    Json(payload): Json<ResponseMusic>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = query(
        r#"INSERT INTO mariage_music (sugest_by, musics) VALUES ($1, $2) RETURNING id"#,
    )
    .bind(payload.sugest_by)
    .bind(payload.musics)
    .map(|row: sqlx::postgres::PgRow| ResponseId { id: row.get(0) })
    .fetch_one(&mut *_conn)
    .await;

    let visit: ResponseId = match &result {
        Ok(row) => row.clone(),
        Err(_) => return Err((StatusCode::BAD_REQUEST, (-1).to_string()))
    };
    Ok((
        StatusCode::OK,
        visit.id.to_string()
    ))
}
