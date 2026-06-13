use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new().route("/ping", get(ping))
}

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn health_check() -> &'static str {
    "ok"
}
