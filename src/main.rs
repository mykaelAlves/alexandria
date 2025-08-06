use std::error::Error;

use alexandria::{
    log::{debug, info, warn},
    responses::{SERVER_CLOSED, SERVER_STARTED},
};

struct AppGlobalState;

impl AppGlobalState {
    fn setup() {
        info(SERVER_STARTED);

        dotenvy::dotenv().ok();
    }
}

impl Drop for AppGlobalState {
    fn drop(&mut self) {
        warn(SERVER_CLOSED);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = AppGlobalState::setup();

    println!("Hello, world!");

    Ok(())
}
