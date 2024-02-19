use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}
