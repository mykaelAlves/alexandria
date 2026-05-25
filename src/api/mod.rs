use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::api::health::ping;

pub mod middleware;
pub mod health;

pub fn create_router() -> Router {
    axum::Router::new()
        .merge(health::create_router())
        .layer(TraceLayer::new_for_http())
}
