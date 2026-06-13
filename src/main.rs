use alexandria::{api, config::AlexandriaConfig};
use tokio::net::TcpListener;
use tracing::{debug, error, info, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    EnvFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

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

    let config = match AlexandriaConfig::new() {
        Ok(c) => c,
        Err(e) => {
            error!("Falha ao carregar configuração: {}", e);
            std::process::exit(1);
        },
    };

    debug!("Configuração carregada: {:#?}", config);

    let app = api::create_router();

    let listener =
        TcpListener::bind(config.server_addr).await.map_err(|e| {
            error!(
                "Falha ao vincular ao endereço {}: {}",
                config.server_addr, e
            );
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
