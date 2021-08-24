mod logging;

use std::env;
use log::{info};

fn init() {
    logging::log_initializer::log_init();
}

#[tokio::main]
async fn main() {
    init();

    info!("Hello, world!");
}
