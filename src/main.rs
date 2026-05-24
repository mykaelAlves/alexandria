use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::{
    trace::TraceLayer,
};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    EnvFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

const SERVER_ADDR: &str = "127.0.0.1:8888";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Iniciando Alexandria...");

    let app = Router::new()
        .route("/ping", get(ping))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(SERVER_ADDR).await?;

    info!("Alexandria rodando em http://{}", SERVER_ADDR);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Falha ao registrar sinal de shutdown (Ctrl+C)");

    info!("Sinal de shutdown recebido. Encerrando Alexandria...");
}

async fn ping() -> &'static str {
    "pong"
}
