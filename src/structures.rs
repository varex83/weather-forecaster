use std::fmt::{Display, Formatter};

/// Error type for the project
#[derive(Clone, PartialEq, Debug)]
pub enum ProjectError {
    /// Error returned when location is not found
    LocationNotFound,
    /// Error returned when no data provider is found
    NoProviderFound,
    /// Serialization error
    SerializationError,
    /// Error writing to file
    FileWritingError,
    /// Error returned when the creation of the directory fails
    DirectoryCreationError,
    /// Error returned when the config file is not found
    NoConfigPathProvided,
    /// Error returned when parsing ENV fails
    EnvConfigError(String),
    /// Error returned when parsing API response fails
    ParsingError,
    /// Error returned when no command is provided
    NoCommandProvided,
    /// Error returned when the API key is not set for the provider
    NoApiKeyForProvider(String),
    /// Custom error
    Custom(String),
}

impl<T> From<T> for ProjectError
where
    T: ToString,
{
    fn from(e: T) -> Self {
        ProjectError::Custom(e.to_string())
    }
}

#[derive(Default, Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ProviderType {
    /// OpenWeather
    #[default]
    OpenWeather,
    /// WeatherApi
    WeatherApi,
}

impl From<String> for ProviderType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "" => ProviderType::default(),
            "openweather" => ProviderType::OpenWeather,
            "weatherapi" => ProviderType::WeatherApi,
            _ => panic!("Unknown provider"),
        }
    }
}

#[derive(Default, Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name.to_string().fmt(f)
    }
}

#[derive(Debug)]
pub struct WeatherApiResponse {
    pub location: Option<Location>,
    pub sunrise: Option<u64>,
    pub sunset: Option<u64>,
    pub temp: Option<f32>,
    pub feels_like: Option<f32>,
    pub pressure: Option<u16>,
    pub humidity: Option<u8>,
}

impl ToString for WeatherApiResponse {
    fn to_string(&self) -> String {
        let mut s = String::new();

        if let Some(location) = &self.location {
            s.push_str(&format!("Weather for Location: {}\n", location));
        }
        if let Some(sunrise) = &self.sunrise {
            s.push_str(&format!(
                "Sunrise: {}\n",
                chrono::NaiveDateTime::from_timestamp_opt(*sunrise as i64, 0).unwrap()
            ));
        }
        if let Some(sunset) = &self.sunset {
            s.push_str(&format!(
                "Sunset: {}\n",
                chrono::NaiveDateTime::from_timestamp_opt(*sunset as i64, 0).unwrap()
            ));
        }
        if let Some(temp) = &self.temp {
            s.push_str(&format!("Temperature: {}C\n", temp));
        }
        if let Some(feels_like) = &self.feels_like {
            s.push_str(&format!("Feels like: {}C\n", feels_like));
        }
        if let Some(pressure) = &self.pressure {
            s.push_str(&format!("Pressure: {}mb\n", pressure));
        }
        if let Some(humidity) = &self.humidity {
            s.push_str(&format!("Humidity: {}%\n", humidity));
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_from_string() {
        let provider = ProviderType::from("openweather".to_string());
        assert_eq!(provider, ProviderType::OpenWeather);
    }

    #[test]
    #[should_panic]
    fn test_provider_from_string_panic() {
        let provider = ProviderType::from("unknown".to_string());
        assert_eq!(provider, ProviderType::OpenWeather);
    }

    #[test]
    fn test_provider_default() {
        let provider = ProviderType::default();
        assert_eq!(provider, ProviderType::OpenWeather);
    }

    #[test]
    fn test_provider_from_string_2() {
        let provider = ProviderType::from("weatherapi".to_string());
        assert_eq!(provider, ProviderType::WeatherApi);
    }
}
