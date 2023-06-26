use crate::structures;
use crate::structures::{Location, ProjectError, WeatherApiResponse};
use crate::traits::Provider;

pub struct OpenWeather {
    api_key: String,
}

impl OpenWeather {
    pub fn new(api_key: String) -> Self {
        OpenWeather { api_key }
    }

    pub fn get_location(&self, location: String) -> Result<Location, ProjectError> {
        let url = format!(
            "https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
            location, self.api_key
        );

        let resp = reqwest::blocking::get(url)?.json::<Vec<Location>>()?;

        if resp.is_empty() {
            return Err(ProjectError::LocationNotFound);
        }

        Ok(resp[0].clone())
    }

    /// ## Dev:
    /// Shouldn't work if you don't have a paid account
    fn get_historical_weather(
        &self,
        loc: Location,
        time: &str,
    ) -> Result<WeatherApiResponse, ProjectError> {
        let url = format!(
            "https://history.openweathermap.org/data/3.0/history/timemachine?lat={}&lon={}&dt={}&appid={}&units=metric",
            loc.lat, loc.lon, {
                let dt = chrono::DateTime::parse_from_rfc3339(time)?;
                dt.timestamp().to_string()
            }, self.api_key
        );

        let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;

        let weather = WeatherApiResponse {
            location: Some(loc),
            sunrise: Some(
                resp["data"]["sunrise"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)?,
            ),
            sunset: Some(
                resp["data"]["sunset"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)?,
            ),
            temp: Some(
                resp["data"]["temp"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            ),
            feels_like: Some(
                resp["data"]["feels_like"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            ),
            pressure: Some(
                resp["data"]["pressure"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)? as u16,
            ),
            humidity: Some(
                resp["data"]["humidity"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)? as u8,
            ),
        };

        Ok(weather)
    }

    fn get_current_weather(&self, loc: Location) -> Result<WeatherApiResponse, ProjectError> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
            loc.lat, loc.lon, self.api_key
        );

        let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;

        let weather = WeatherApiResponse {
            location: Some(loc),
            sunrise: Some(
                resp["sys"]["sunrise"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)?,
            ),
            sunset: Some(
                resp["sys"]["sunset"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)?,
            ),
            temp: Some(
                resp["main"]["temp"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            ),
            feels_like: Some(
                resp["main"]["feels_like"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            ),
            pressure: Some(
                resp["main"]["pressure"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)? as u16,
            ),
            humidity: Some(
                resp["main"]["humidity"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)? as u8,
            ),
        };

        Ok(weather)
    }
}

impl Provider for OpenWeather {
    fn get_weather(
        &self,
        location: &str,
        time: Option<&str>,
    ) -> Result<Box<dyn ToString>, ProjectError> {
        let loc = self.get_location(location.to_string())?;

        let weather = match time {
            Some(time) => self.get_historical_weather(loc, time)?,
            None => self.get_current_weather(loc)?,
        };

        Ok(Box::new(weather))
    }

    fn get_type(&self) -> structures::ProviderType {
        structures::ProviderType::OpenWeather
    }
}
