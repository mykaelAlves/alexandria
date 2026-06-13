use axum::{
    Router,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use tower_http::trace::TraceLayer;
use tracing::debug;

pub mod health;
pub mod middleware;

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    debug!(
        "Tentativa de acesso a rota não mapeada/implementada: {}",
        uri.path()
    );

    (StatusCode::NOT_FOUND, "Página não encontrada")
}

pub fn create_router() -> Router {
    axum::Router::new()
        .merge(health::create_router())
        .fallback(fallback_handler)
        .layer(TraceLayer::new_for_http())
}
