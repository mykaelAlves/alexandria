use std::error::Error;

use alexandria::app::ServerApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let server_app = ServerApp::new().await?;

	server_app.run().await?;
	server_app.shutdown().await;

	Ok(())
}
