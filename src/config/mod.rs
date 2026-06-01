use std::net::SocketAddrV4;

use config::{Case, Config, Environment};
use serde::Deserialize;
use tracing::{debug, error};

#[derive(Debug, Deserialize)]
pub struct AlexandriaConfig {
    pub server_addr: SocketAddrV4,
    pub database_url: String,
    pub secret_key: String,
}

impl AlexandriaConfig {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        let config = match Config::builder()
            .add_source(
                Environment::with_prefix("ALEXANDRIA")
                    .try_parsing(true)
                    .separator("__")
                    .convert_case(Case::Snake),
            )
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                error!("Falha ao carregar configuração: {}", e);
                std::process::exit(1);
            },
        };

        debug!("Configuração carregada: {:#?}", config);

        match config.try_deserialize() {
            Ok(c) => return c,
            Err(e) => {
                error!("Falha ao deserializar configuração: {}", e);
                std::process::exit(1);
            },
        }
    }
}
