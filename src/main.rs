use alexandria::api;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{error, info, level_filters::LevelFilter, warn};
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

    let app = api::create_router();

    let listener = TcpListener::bind(SERVER_ADDR).await.map_err(|e| {
        error!("Falha ao vincular ao endereço {}: {}", SERVER_ADDR, e);
        Box::<dyn std::error::Error>::from(e)
    })?;

    info!("Alexandria rodando em http://{}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            error!("Falha ao iniciar o servidor: {}", e);
            Box::<dyn std::error::Error>::from(e)
        })?;

    Ok(())
}

async fn shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => info!("Sinal de shutdown recebido. Encerrando Alexandria..."),
        Err(e) => warn!("Falha ao registrar sinal de shutdown: {}", e),
    }
}
