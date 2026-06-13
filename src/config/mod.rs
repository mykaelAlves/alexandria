use std::net::SocketAddrV4;

use config::{Case, Config, ConfigError, Environment};
use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AlexandriaConfig {
    pub server_addr: SocketAddrV4,
    pub database_url: SecretString,
    pub secret_key: SecretString,
}

impl AlexandriaConfig {
    pub fn new() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let config = Config::builder()
            .add_source(
                Environment::with_prefix("ALEXANDRIA")
                    .try_parsing(true)
                    .separator("__")
                    .convert_case(Case::Snake),
            )
            .build()?;

        config.try_deserialize()
    }
}
