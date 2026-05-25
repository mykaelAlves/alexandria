use axum::{Router, routing::get};
use tracing::warn;

pub fn create_router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/health", get(health_check))
}

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn health_check() -> &'static str {
    warn!("Rota não implementada foi acessada: /health");
    "ok"
}