use std::env;

use env_logger;
use log::info;

const RUST_LOG_KEY: &'static str = "RUST_LOG";

pub fn log_init() {
    match env::var(RUST_LOG_KEY) {
        Ok(_) => {
            info!("action=log_init, message=RUST_LOG env variable found");
        }
        Err(_) => {
            info!("action=log_init, message=RUST_LOG env variable not found, setting to default INFO");
            env::set_var(RUST_LOG_KEY, "INFO");
        }
    }

    env_logger::init();
    info!("action=log_init, message=logger initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_init_env_set() {
        super::log_init();
        let expected = String::from("INFO");
        let actual = env::var(RUST_LOG_KEY).unwrap();
        assert_eq!(expected, actual);
    }
}