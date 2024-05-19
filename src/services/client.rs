pub struct Context {
    pub open_weather_url: String,
}

impl Context {
    const OPEN_WEATHER_DEFAULT_SERVER: &'static str = "https://api.openweathermap.org/data/2.5/";

    pub fn new() -> Self {
        Self {
            open_weather_url: String::from(Self::OPEN_WEATHER_DEFAULT_SERVER),
        }
    }
}
