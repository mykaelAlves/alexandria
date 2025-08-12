use std::error::Error;

use alexandria::{
    config::SERVER_APP,
    log::err,
    responses::FAILED_CREATE_SERVER,
    run::{ServerApp, ServerGuard},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _guard = ServerGuard;

    SERVER_APP.run().await?;
    SERVER_APP.shutdown().await;

    Ok(())
}
