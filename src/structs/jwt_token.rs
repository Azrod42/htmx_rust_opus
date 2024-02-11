use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtToken {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
