use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
