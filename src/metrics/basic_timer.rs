pub mod basic_timer {
    use std::time::{Duration, SystemTime};

    use crate::metrics::client::client::call_once;

    pub fn single_api_call(uri: &String) -> Duration {
        let start = SystemTime::now();
        call_once(uri);
        let total_execution_time = SystemTime::now().duration_since(start).expect("Error in system time");
        log::info!("action=single_api_call, duration={:?}", total_execution_time);
        total_execution_time
    }
}

#[cfg(test)]
mod tests {
    use crate::metrics::basic_timer::basic_timer::single_api_call;

    #[test]
    fn test_single_api_call_valid_uri() {
        let valid_uri = String::from("http://reddit.com/");
        single_api_call(&valid_uri);
    }
}