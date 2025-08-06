pub mod models;
pub mod handlers;

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

    pub fn err(msg: &str, err: Box<dyn Error>) -> Result<(), Box<dyn Error>> {
        let time = get_time_now();
        let msg = format!("[{}][{time}] ({err}) {msg}", "ERRO".bright_red());
        eprintln!("{msg}");
        write_to_log(&msg);

        Err(err)
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

    pub const CONFIG_PATH: &str = "config/config.ron";
    pub const DEFAULT_LOGGING_PATH: &str = "server.log";
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

    pub const FAILED_CREATE_LISTENER: &str =
        "Houve um erro ao criar o listener!!";
    pub const FAILED_CREATE_SERVER: &str =
        "Houve um erro ao criar o servidor!!";
}

pub mod run {
    use crate::{log::err, responses::SERVER_RUNNING};
    use crate::handlers;
    use axum::Router;
    use axum::routing::any;
    use serde::Deserialize;
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

    pub struct GlobalState;

    impl GlobalState {
        fn new() -> Result<Self, Box<dyn Error>> {
            Ok(Self {})
        }
    }

    pub struct ServerApp {
        is_running: bool,
        state: GlobalState,
        config: Config,
    }

    impl ServerApp {
        pub fn new() -> Result<Self, Box<dyn Error>> {
            info(SERVER_STARTED);

            let config = Config::new()?;
            let state = GlobalState::new()?;

            Ok(Self {
                is_running: true,
                state,
                config,
            })
        }

        pub async fn run(&self) -> Result<(), Box<dyn Error>> {
            let app: Router =
                axum::Router::new().route("/", any(handlers::root));

            let listener = match TcpListener::bind(self.get_ipv4()).await {
                Ok(l) => l,
                Err(e) => return err(FAILED_CREATE_LISTENER, Box::new(e)),
            };

            info(SERVER_RUNNING);

            axum::serve(listener, app.into_make_service()).await?;

            Ok(())
        }

        pub async fn shutdown(&mut self) {
            warn(SERVER_CLOSED);

            self.is_running = false;
        }

        pub fn get_ipv4(&self) -> SocketAddrV4 {
            self.config.network.ip
        }
    }

    impl Drop for ServerApp {
        fn drop(&mut self) {
            if self.is_running {
                warn(SERVER_CLOSED_WRONGLY);
            }

            self.is_running = false;
        }
    }
}
