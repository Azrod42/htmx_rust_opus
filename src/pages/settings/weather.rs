use chrono::{NaiveDateTime, Utc};
use sqlx::{query, Row};

use crate::{
    services::client::Context,
    structs::{
        database::DatabaseConnection,
        entity::{
            user::User,
            weather::{RawWeather, Weather},
        },
    },
};

fn is_older_than_10_minutes(naive_datetime: NaiveDateTime) -> bool {
    let now = Utc::now().naive_utc();
    let ten_minutes = chrono::Duration::minutes(10);
    let limit = now - ten_minutes;
    naive_datetime < limit
}

pub async fn get_user_weather(
    user: User,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Option<Weather> {
    let res = sqlx::query("select temp, feels_like, humidity, weather_main, weather_description, weather_icon, city, timezone, date from weather WHERE user_id = $1 ORDER BY ID DESC LIMIT 1")
        .bind(&user.id)
        .map(|row: sqlx::postgres::PgRow| Weather {
            temp: row.get(0),
            feels_like: row.get(1),
            humidity: row.get(2),
            weather_main: row.get(3),
            weather_description: row.get(4),
            weather_icon: row.get(5),
            city: row.get(6),
            timezone: row.get(7),
            date: row.get(8),
            ..Default::default()
        })
        .fetch_one(&mut *conn)
        .await
        .unwrap_or(Weather {
            ..Default::default()
        });

    if !is_older_than_10_minutes(res.date) {
        return Some(res);
    }

    let res = sqlx::query("SELECT lon, lat, open_weather_api_key FROM users WHERE email = $1")
        .bind(&user.email)
        .map(|row: sqlx::postgres::PgRow| User {
            lon: row.get(0),
            lat: row.get(1),
            open_weather_api_key: row.get(2),
            ..Default::default()
        })
        .fetch_one(&mut *conn)
        .await
        .unwrap_or(User {
            ..Default::default()
        });

    let context = Context::new();
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "{}{}?lat={}&lon={}&appid={}&units=metric",
            context.open_weather_url, "/weather", res.lat, res.lon, res.open_weather_api_key
        ))
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        let res: RawWeather = res.json().await.unwrap();
        let weather = Weather {
            id: res.id,
            city: res.name,
            feels_like: res.main.feels_like.to_string(),
            temp: res.main.temp.to_string(),
            humidity: res.main.humidity.to_string(),
            timezone: res.timezone,
            weather_main: res.weather[0].main.clone(),
            weather_description: res.weather[0].description.clone(),
            weather_icon: res.weather[0].icon.clone(),
            ..Default::default()
        };

        let _ = query!(
        r#"INSERT INTO weather (user_id, temp, feels_like, humidity, weather_main, weather_description, weather_icon, city,timezone, counrty) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id"#,
            user.id, weather.temp, weather.feels_like, weather.humidity, weather.weather_main, weather.weather_description, weather.weather_icon, weather.city, weather.timezone, String::new()
    )
    .fetch_one(&mut *conn)
    .await;
        return Some(weather);
    };
    None
}
