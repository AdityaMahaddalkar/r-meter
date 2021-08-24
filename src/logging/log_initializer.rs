use std::env;

use env_logger;
use log::{info};
use std::env::VarError;

pub fn log_init() {
    match env::var("RUST_LOG") {
        Ok(_) => {
            info!("action=log_init, message=RUST_LOG env variable found");
        }
        Err(_) => {
            info!("action=log_init, message=RUST_LOG env variable not found, setting to default INFO");
            env::set_var("RUST_LOG", "INFO");
        }
    }

    env_logger::init();
    info!("action=log_init, message=logger initialized");
}