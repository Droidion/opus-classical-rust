use sqlx::postgres::{PgConnectOptions, PgSslMode};

/// Application configuration

/// All settings
#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub static_assets_url: String,
    pub umami_id: String,
}

/// Web server specific settings
#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
}

/// Database specific settings
#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

/// Reads configuration from a set number of external sources.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        // Read from environments variables
        // Example: `APP_APPLICATION__PORT=5001` would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    /// Creates Postgres settings.
    pub fn get_connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
    }
}
