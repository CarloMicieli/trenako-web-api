use config::{Config, Environment, File};
use serde::Deserialize;

/// Application settings
#[derive(Deserialize, Debug)]
pub struct Settings {
    database: DatabaseSettings,
    server: ServerSettings,
}

impl Settings {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    pub fn get_database_url(&self) -> String {
        self.database.database_url()
    }

    pub fn database_settings(&self) -> &DatabaseSettings {
        &self.database
    }

    pub fn load() -> Result<Settings, config::ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/application").required(false))
            .add_source(
                Environment::default().separator("_").ignore_empty(true),
            )
            .build()?;
        s.try_deserialize()
    }
}

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    host: String,
    port: u16,
}

/// Database settings
#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    username: String,
    password: String,
    host: String,
    port: u16,
    name: String,
}

impl DatabaseSettings {
    pub fn new(
        username: &str,
        password: &str,
        host: &str,
        port: u16,
        name: &str,
    ) -> DatabaseSettings {
        DatabaseSettings {
            username: username.to_owned(),
            password: password.to_owned(),
            host: host.to_owned(),
            port,
            name: name.to_owned(),
        }
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgresql://{username}:{password}@{host}:{port}/{name}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
            name = self.name
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
