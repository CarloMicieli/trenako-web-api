use config::{Config, Environment, File};
use serde::Deserialize;

/// Application settings
#[derive(Deserialize, Debug)]
pub struct Settings {
    database: DatabaseSettings,
    app_port: u16,
    secret_key: String,
}

impl Settings {
    pub fn get_address(&self) -> String {
        format!("0.0.0.0:{}", self.app_port)
    }

    pub fn get_database_url(&self) -> String {
        self.database.database_url()
    }

    pub fn database_settings(&self) -> &DatabaseSettings {
        &self.database
    }

    pub fn load() -> Result<Settings, config::ConfigError> {
        let mut settings = Config::default();
        settings.merge(File::with_name("application"))?;
        settings.merge(Environment::new())?;
        settings.try_into()
    }
}

/// Database settings
#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    username: String,
    password: String,
    host: String,
    port: u16,
    database_name: String,
}

impl DatabaseSettings {
    pub fn new(
        username: &str,
        password: &str,
        host: &str,
        port: u16,
        database_name: &str,
    ) -> DatabaseSettings {
        DatabaseSettings {
            username: username.to_owned(),
            password: password.to_owned(),
            host: host.to_owned(),
            port,
            database_name: database_name.to_owned(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgresql://{username}:{password}@{host}:{port}/{database_name}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
            database_name = self.database_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_database_urls() {
        let settings = DatabaseSettings::new(
            "postgres",
            "pa$$word",
            "database-host",
            5432,
            "database-name",
        );

        assert_eq!(
            "postgresql://postgres:pa$$word@database-host:5432/database-name",
            settings.database_url()
        );
    }
}
