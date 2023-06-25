use crate::structures;
use crate::structures::{Location, ProjectError, WeatherApiResponse};
use crate::traits::Provider;

pub struct WeatherApi {
    api_key: String,
}

impl WeatherApi {
    pub fn new(api_key: String) -> Self {
        WeatherApi { api_key }
    }

    /// ## Dev:
    /// Shouldn't work if you don't have a paid account
    fn get_historical_weather(
        &self,
        loc: String,
        time: &str,
    ) -> Result<WeatherApiResponse, ProjectError> {
        let url = format!(
            "http://api.weatherapi.com/v1/history.json?key={}&q={}&dt={}",
            self.api_key, loc, time
        );

        let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;

        let weather = WeatherApiResponse {
            location: Some(Location {
                name: resp["location"]["name"]
                    .as_str()
                    .ok_or(ProjectError::ParsingError)?
                    .to_string(),
                lat: resp["location"]["lat"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
                lon: resp["location"]["lon"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            }),
            sunrise: None,
            sunset: None,
            temp: Some(
                resp["hour"]["temp_c"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            ),
            feels_like: Some(
                resp["hour"]["feelslike_c"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            ),
            pressure: Some(
                resp["hour"]["pressure_mb"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as u16,
            ),
            humidity: Some(
                resp["hour"]["humidity"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)? as u8,
            ),
        };

        Ok(weather)
    }

    fn get_current_weather(&self, loc: String) -> Result<WeatherApiResponse, ProjectError> {
        let url = format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            self.api_key, loc
        );

        let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;

        let weather = WeatherApiResponse {
            location: Some(Location {
                name: resp["location"]["name"]
                    .as_str()
                    .ok_or(ProjectError::ParsingError)?
                    .to_string(),
                lat: resp["location"]["lat"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
                lon: resp["location"]["lon"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            }),
            sunrise: None,
            sunset: None,
            temp: Some(
                resp["current"]["temp_c"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            ),
            feels_like: Some(
                resp["current"]["feelslike_c"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as f32,
            ),
            pressure: Some(
                resp["current"]["pressure_mb"]
                    .as_f64()
                    .ok_or(ProjectError::ParsingError)? as u16,
            ),
            humidity: Some(
                resp["current"]["humidity"]
                    .as_u64()
                    .ok_or(ProjectError::ParsingError)? as u8,
            ),
        };

        Ok(weather)
    }
}

impl Provider for WeatherApi {
    fn get_weather(
        &self,
        loc: &str,
        time: Option<&str>,
    ) -> Result<Box<dyn ToString>, ProjectError> {
        let weather = match time {
            Some(time) => self.get_historical_weather(loc.to_string(), time)?,
            None => self.get_current_weather(loc.to_string())?,
        };

        Ok(Box::new(weather))
    }

    fn get_type(&self) -> structures::ProviderType {
        structures::ProviderType::WeatherApi
    }
}
