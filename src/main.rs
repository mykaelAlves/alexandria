use std::error::Error;

use alexandria::{
    log::err,
    responses::FAILED_CREATE_SERVER,
    run::ServerApp,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut server = match ServerApp::new() {
        Ok(st) => st,
        Err(e) => return err(FAILED_CREATE_SERVER, e),
    };

    server.run().await?;

    server.shutdown().await;

    Ok(())
}
