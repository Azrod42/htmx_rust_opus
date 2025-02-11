use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub lon: f32,
    pub lat: f32,
    pub open_weather_api_key: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            username: String::new(),
            email: String::new(),
            password: String::new(),
            lon: 0.0,
            lat: 0.0,
            open_weather_api_key: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserId {
    pub id: i32,
}
