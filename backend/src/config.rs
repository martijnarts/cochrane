use getset::Getters;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml).required(false),
        )
        .add_source(config::Environment::default().separator("__"))
        .build()?;
    settings.try_deserialize::<Config>()
}

#[derive(Deserialize, Getters)]
pub struct Config {
    #[getset(get = "pub")]
    database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    username: SecretString,
    password: SecretString,
    port: u16,
    host: String,
    database_name: String,
}

impl DatabaseConfig {
    pub fn as_connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(self.username.expose_secret())
            .password(self.password.expose_secret())
            .port(self.port)
            .database(&self.database_name)
    }
}
