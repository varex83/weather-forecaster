use crate::structures::{ProjectError, ProviderType};
use std::fs;
use std::path::Path;

impl ProviderType {
    pub fn serialize_to_config_file(&self, config_file_path: &str) -> Result<(), ProjectError> {
        let serialized_provider =
            serde_json::to_string(&self).map_err(|_| ProjectError::SerializationError)?;

        let path = Path::new(config_file_path);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).map_err(|_| ProjectError::DirectoryCreationError)?;
        }

        fs::write(path, serialized_provider).map_err(|_| ProjectError::FileWritingError)?;

        Ok(())
    }

    pub fn deserialize_from_config_file(config_file_path: &str) -> Result<Self, ProjectError> {
        let data = fs::read_to_string(config_file_path)?;

        let deserialized_provider: Self = serde_json::from_str(&data).unwrap_or_default();

        Ok(deserialized_provider)
    }
}
