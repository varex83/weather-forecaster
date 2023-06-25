mod env;
mod provider_serde;
mod providers;
mod structures;
mod traits;

use crate::structures::{ProjectError, ProviderType};
use crate::traits::Provider;
use clap::{Parser, Subcommand};

/// Weather reporting CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, PartialEq, Debug)]
enum Commands {
    /// does testing things
    Configure {
        /// Configure the data provider: (openweather - default, weatherapi)
        #[arg(short, long)]
        provider: ProviderType,
    },
    Get {
        /// Location to get weather for
        #[arg(short, long)]
        location: String,
        /// Time (default: now)
        /// Format: YYYY-MM-DDTHH:MM:SS
        #[arg(short, long)]
        time: Option<String>,
    },
}

fn main() -> Result<(), ProjectError> {
    let cli = Cli::parse();

    let env_config = env::EnvConfig::new()?;

    let providers: Vec<Box<dyn Provider>> = vec![
        Box::new(providers::OpenWeather::new(
            env_config
                .open_weather_api_key
                .ok_or(ProjectError::NoApiKeyForProvider(
                    "No Open Weather API Key".to_string(),
                ))?,
        )),
        Box::new(providers::WeatherApi::new(
            env_config
                .weather_api_api_key
                .ok_or(ProjectError::NoApiKeyForProvider(
                    "No Weather API API Key".to_string(),
                ))?,
        )),
    ];

    match cli.command {
        Some(Commands::Configure { provider }) => {
            let path = env_config
                .config_path
                .ok_or(ProjectError::NoConfigPathProvided)?;

            provider.serialize_to_config_file(path.as_str())?;

            println!("Provider {provider:?} successfully saved to {path}");

            Ok(())
        }
        Some(Commands::Get { location, time }) => {
            let provider = ProviderType::deserialize_from_config_file(
                env_config.config_path.unwrap_or_default().as_str(),
            )?;

            let provider = providers
                .iter()
                .find(|p| p.get_type() == provider)
                .ok_or(ProjectError::NoProviderFound)?;

            println!("Using provider: {:?}", provider.get_type());

            let weather = provider.get_weather(location.as_str(), time.as_deref())?;

            println!("{}", weather.to_string());

            Ok(())
        }
        None => Err(ProjectError::NoCommandProvided),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_configure() {
        let cli = Cli::parse_from(&["weather", "configure", "--provider", "weatherapi"]);
        assert_eq!(
            cli.command,
            Some(Commands::Configure {
                provider: ProviderType::WeatherApi
            })
        );
    }

    #[test]
    fn test_cli_run() {
        let cli = Cli::parse_from(&[
            "weather",
            "get",
            "--location",
            "London",
            "--time",
            "2021-01-01T12:00:00",
        ]);
        assert_eq!(
            cli.command,
            Some(Commands::Get {
                location: "London".to_string(),
                time: Some("2021-01-01T12:00:00".to_string())
            })
        );
    }
}
