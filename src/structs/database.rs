use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub async fn init_database() -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("can't connect to database")
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
