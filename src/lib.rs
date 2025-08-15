pub mod handlers;
pub mod models;
pub mod services;

pub mod log {
    use chrono::prelude::*;
    use colored::Colorize;
    use std::error::Error;

    const TIMEZONE_OFFSET_SECONDS: i32 = -3 * 3600;

    fn get_time_now() -> String {
        let time = Utc::now();
        let fixed_offset =
            FixedOffset::east_opt(TIMEZONE_OFFSET_SECONDS).unwrap();

        time.with_timezone(&fixed_offset)
            .format("%d/%m/%Y %H:%M:%S")
            .to_string()
    }

    pub fn info(msg: &str) {
        let time = get_time_now();
        println!("[{}][{time}] {msg}", "INFO".bright_cyan());
    }

    pub fn warn(msg: &str) {
        let time = get_time_now();
        let msg = format!("[{}][{time}] {msg}", "AVISO".yellow());
        println!("{msg}");
        write_to_log(&msg);
    }

    pub fn err(msg: &str, err: Box<dyn Error>) -> Box<dyn Error> {
        let time = get_time_now();
        let msg = format!("[{}][{time}] ({err}) {msg}", "ERRO".bright_red());
        eprintln!("{msg}");
        write_to_log(&msg);

        err
    }

    pub fn debug(msg: &str) {
        let time = get_time_now();
        let msg = format!("[DEBUG][{time}] {msg}").bright_black();
        eprintln!("{msg}");
    }

    fn write_to_log(msg: &str) {}
}

pub mod config {
    use std::sync::{LazyLock, Mutex};

    use crate::run::ServerApp;

    pub const CONFIG_PATH: &str = "config/config.ron";
    const DEFAULT_LOGGING_PATH: &str = "server.log";
    pub static LOGGING_PATH: LazyLock<Mutex<String>> =
        LazyLock::new(|| Mutex::new(String::from(DEFAULT_LOGGING_PATH)));
}

pub mod responses {
    pub const SERVER_STARTED: &str = "O servidor foi ligado.";
    pub const SERVER_RUNNING: &str = "O servidor está aberto a requisições...";
    pub const SERVER_CLOSED: &str = "O servidor foi desligado!";
    pub const SERVER_CLOSED_WRONGLY: &str =
        "O servidor foi encerrado incorretamente!";

    pub const CONFIG_READING: &str = "Lendo configuração do servidor...";

    pub const DATABASE_CONNECTING: &str =
        "Conectando ao banco de dados...";

    pub const FAILED_CREATE_LISTENER: &str =
        "Houve um erro ao criar o listener!!";
    pub const FAILED_CREATE_SERVER: &str =
        "Houve um erro ao criar o servidor!!";
    pub const FAILED_DATABASE_CONNECTION: &str =
        "Houve um erro ao conectar ao banco de dados!!";
}

pub mod run {
    use crate::handlers;
    use crate::responses::{DATABASE_CONNECTING, FAILED_DATABASE_CONNECTION};
    use crate::{log::err, responses::SERVER_RUNNING};
    use axum::Router;
    use axum::routing::any;
    use serde::Deserialize;
    use sqlx::postgres;
    use std::sync::atomic::AtomicBool;
    use std::{error::Error, net::SocketAddrV4};
    use tokio::net::TcpListener;

    use crate::{
        config::{CONFIG_PATH, LOGGING_PATH},
        log::{debug, info, warn},
        responses::{
            CONFIG_READING, FAILED_CREATE_LISTENER, SERVER_CLOSED,
            SERVER_CLOSED_WRONGLY, SERVER_STARTED,
        },
    };

    #[derive(Deserialize)]
    struct Database {
        url: String,
        pool: Pool,
    }

    #[derive(Deserialize)]
    struct Pool {
        max_connections: u8,
        connect_timeout_seconds: u16,
    }

    #[derive(Deserialize)]
    struct Network {
        ip: SocketAddrV4,
    }

    #[derive(Deserialize)]
    struct Auth {
        jwt_secret: String,
        jwt_expiration_in_min: u16,
        jwt_issuer: String,
    }

    #[derive(Deserialize)]
    struct Config {
        network: Network,
        database: Database,
        logging_path: String,
        auth: Auth,
    }

    impl Config {
        fn new() -> Result<Self, Box<dyn Error>> {
            info(CONFIG_READING);

            let s = std::fs::read_to_string(CONFIG_PATH)?;

            #[cfg(debug_assertions)]
            debug(&s);

            let config: Self = ron::from_str(&s)?;
            let mut logging_path_lock = LOGGING_PATH.lock().unwrap();
            logging_path_lock.clear();
            logging_path_lock.push_str(&config.logging_path);

            Ok(config)
        }
    }

    #[derive(Clone)]
    struct GlobalState {
        pg_pool: postgres::PgPool,
    }

    impl GlobalState {
        async fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
            info(DATABASE_CONNECTING);

            let pg_pool = match postgres::PgPoolOptions::new()
                .max_connections(config.database.pool.max_connections as u32)
                .connect(&config.database.url)
                .await
            {
                Ok(pg_pool) => pg_pool,
                Err(e) => {
                    let e = err(FAILED_DATABASE_CONNECTION, Box::new(e));

                    return Err(e);
                }
            };

            Ok(Self { pg_pool })
        }
    }

    pub struct ServerApp {
        is_running: AtomicBool,
        state: GlobalState,
        config: Config,
    }

    impl ServerApp {
        pub async fn new() -> Result<Self, Box<dyn Error>> {
            info(SERVER_STARTED);

            let config = Config::new()?;
            let state = GlobalState::new(&config).await?;

            Ok(Self {
                is_running: AtomicBool::new(true),
                state,
                config,
            })
        }

        pub async fn run(&self) -> Result<(), Box<dyn Error>> {
            let app: Router = axum::Router::new()
                .route("/", any(handlers::root))
                .with_state(self.state.clone());

            let listener = match TcpListener::bind(self.config.network.ip).await
            {
                Ok(l) => l,
                Err(e) => return Err(err(FAILED_CREATE_LISTENER, Box::new(e))),
            };

            info(SERVER_RUNNING);

            axum::serve(listener, app.into_make_service()).await?;

            Ok(())
        }

        pub async fn shutdown(&self) {
            warn(SERVER_CLOSED);

            self.is_running
                .store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }

    pub struct ServerGuard;

    impl Drop for ServerApp {
        fn drop(&mut self) {
            if self
                .is_running
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                warn(SERVER_CLOSED_WRONGLY);
            }

            self.is_running
                .store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
