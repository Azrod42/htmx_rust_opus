use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    pub id: i32,
    pub temp: String,
    pub feels_like: String,
    pub humidity: String,
    pub weather_main: String,
    pub weather_description: String,
    pub weather_icon: String,
    pub city: String,
    pub timezone: f32,
    pub date: chrono::NaiveDateTime,
}

impl Default for Weather {
    fn default() -> Self {
        Weather {
            id: 0,
            temp: String::new(),
            feels_like: String::new(),
            humidity: String::new(),
            weather_main: String::new(),
            weather_description: String::new(),
            weather_icon: String::new(),
            city: String::from("no data"),
            timezone: 0.0,
            date: chrono::NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                NaiveTime::from_hms_milli_opt(12, 00, 00, 000).unwrap(),
            ),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawWeatherMain {
    pub temp: f32,
    pub feels_like: f32,
    pub humidity: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawWeatherWeather {
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawWeather {
    pub id: i32,
    pub main: RawWeatherMain,
    pub weather: Vec<RawWeatherWeather>,
    pub name: String,
    pub timezone: f32,
}
