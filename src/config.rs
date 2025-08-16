use std::sync::{LazyLock, Mutex};

use crate::run::ServerApp;

pub const CONFIG_PATH: &str = "config/config.ron";
const DEFAULT_LOGGING_PATH: &str = "server.log";
pub static LOGGING_PATH: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(String::from(DEFAULT_LOGGING_PATH)));
