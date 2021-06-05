use crate::configuration::DatabaseSettings;
use refinery::config::Config;
use thiserror::Error;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/db/migrations");
}

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("connection error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("refinery migration error: {0}")]
    RefineryError(#[from] refinery::Error),
}

pub async fn migrate(
    db_settings: &DatabaseSettings,
) -> Result<(), MigrationError> {
    let mut conn = Config::new(refinery::config::ConfigDbType::Postgres)
        .set_db_host(db_settings.host())
        .set_db_port(&db_settings.port().to_string())
        .set_db_name(db_settings.database_name())
        .set_db_user(db_settings.username())
        .set_db_pass(db_settings.password());

    embedded::migrations::runner().run_async(&mut conn).await?;

    Ok(())
}
