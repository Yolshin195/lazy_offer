use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TelegramSettings {
    pub api_id: String,
    pub api_hash: String,
    pub bot_token: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub telegram: TelegramSettings,
}

impl Settings {

    pub fn new() -> Result<Self, ConfigError> {
        Settings::load(".env")
    }

    pub fn load(file_path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = file_path.as_ref();

        if path.exists() {
            dotenvy::from_path(path).ok(); 
        }

        let s = Config::builder()
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;

        s.try_deserialize()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_consistency() {
        // В тестах передаем путь к примеру. 
        // Если запускаем из apps/main, путь будет "../../.env-example"
        let settings = Settings::load("../../.env-example")
            .expect("Failed to load example config");
        
        assert_eq!(settings.telegram.api_id, "1234567"); // Значение из .env-example
    }
}