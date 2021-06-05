use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    database_url: String,
    app_port: u16,
    secret_key: String,
}

/// Application settings
impl Settings {
    pub fn get_address(&self) -> String {
        format!("0.0.0.0:{}", self.app_port)
    }

    pub fn get_database_url(&self) -> &str {
        &self.database_url
    }

    pub fn load() -> Result<Settings, config::ConfigError> {
        let mut settings = Config::default();
        settings.merge(File::with_name("application"))?;
        settings.merge(Environment::new())?;
        settings.try_into()
    }
}
