#![allow(dead_code)]

use crate::handlers;
use crate::response::{
	DATABASE_CONNECTED, DATABASE_CONNECTING, FAILED_DATABASE_CONNECTION,
};
use crate::{log::err, response::SERVER_RUNNING};
use axum::Router;
use axum::routing::{any, get};
use serde::Deserialize;
use sqlx::postgres;
use std::sync::atomic::AtomicBool;
use std::{error::Error, net::SocketAddrV4};
use tokio::net::TcpListener;

use crate::{
	config::{CONFIG_PATH, LOGGING_PATH},
	log::{debug, info, warn},
	response::{
		CONFIG_READING, FAILED_CREATE_LISTENER, SERVER_CLOSED,
		SERVER_CLOSED_WRONGLY, SERVER_STARTED,
	},
};

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
			.route("/motivo", any(handlers::entities::motivo::root))
			.route("/motivo/list", get(handlers::entities::motivo::list))
			.route("/motivo/get", get(handlers::entities::motivo::get))
			.with_state(self.state.clone());

		let listener = match TcpListener::bind(self.config.network.ip).await {
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

	#[inline]
	pub fn state(&self) -> &GlobalState {
		&self.state
	}
}

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
struct Storage {
	log: String,
	data: String,
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
	storage: Storage,
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
		logging_path_lock.push_str(&config.storage.log);

		Ok(config)
	}
}

#[readonly::make]
#[derive(Clone)]
pub struct GlobalState {
	pub pg_pool: postgres::PgPool,
}

impl GlobalState {
	async fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
		info(DATABASE_CONNECTING);

		let pg_pool = match postgres::PgPoolOptions::new()
			.max_connections(config.database.pool.max_connections as u32)
			.connect(&config.database.url)
			.await
		{
			Ok(pg_pool) => {
				info(DATABASE_CONNECTED);

				pg_pool
			}
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

pub struct ServerGuard;

impl Drop for ServerApp {
	fn drop(&mut self) {
		if self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
			warn(SERVER_CLOSED_WRONGLY);
		}

		self.is_running
			.store(false, std::sync::atomic::Ordering::SeqCst);
	}
}
