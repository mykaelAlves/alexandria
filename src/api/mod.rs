use axum::Router;
use tower_http::trace::TraceLayer;

pub mod health;
pub mod middleware;

pub fn create_router() -> Router {
    axum::Router::new()
        .merge(health::create_router())
        .layer(TraceLayer::new_for_http())
}
