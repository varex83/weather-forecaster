use crate::structures::ProjectError;
use envconfig::Envconfig;

#[derive(Envconfig, Default, Debug, Clone)]
pub struct EnvConfig {
    #[envconfig(from = "CONFIG_PATH")]
    pub config_path: Option<String>,
    #[envconfig(from = "OPEN_WEATHER_API_KEY")]
    pub open_weather_api_key: Option<String>,
    #[envconfig(from = "WEATHER_API_API_KEY")]
    pub weather_api_api_key: Option<String>,
}

impl EnvConfig {
    pub fn new() -> Result<Self, ProjectError> {
        dotenv::dotenv().ok();
        EnvConfig::init_from_env().map_err(|err| ProjectError::EnvConfigError(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_config() {
        std::env::set_var("OPEN_WEATHER_API_KEY", "test");

        let env_config = EnvConfig::new().unwrap();

        assert_eq!(env_config.open_weather_api_key, Some("test".to_string()));
    }
}
