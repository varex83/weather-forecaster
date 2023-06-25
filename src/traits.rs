use crate::structures;
use crate::structures::ProjectError;

/// Trait provider is used to define the interface for weather providers
pub trait Provider {
    fn get_weather(
        &self,
        location: &str,
        time: Option<&str>,
    ) -> Result<Box<dyn ToString>, ProjectError>;
    fn get_type(&self) -> structures::ProviderType;
}
