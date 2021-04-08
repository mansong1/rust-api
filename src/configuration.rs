/*!
src/configuration.rs
*/
use std::convert::{TryFrom, TryInto};
use std::u16;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}
#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    // Initialize config reader
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    /* Detect running environment. Default to 'local' */
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;
    let _environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into() // try converting config values read into our Settings type
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "
                {} is not a supported environment. Use either local or production.",
                other
            )),
        }
    }
}
