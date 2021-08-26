const N_WORKERS: i32 = 8;

pub mod aggregator_timer {
    use std::collections::HashMap;
    use std::sync::mpsc::channel;
    use std::time::SystemTime;

    use threadpool::ThreadPool;

    use crate::metrics::aggregator_timer::N_WORKERS;
    use crate::metrics::client::client::call_once;

    pub fn multiple_api_calls_aggregation(uri: &String, count_of_calls: u32) -> HashMap<String, u128> {
        let millis_times = run_multiple_calls(uri, count_of_calls);

        let mut aggregation_map = HashMap::new();
        let min = millis_times.iter().min().unwrap();
        let max = millis_times.iter().max().unwrap();
        let sum = millis_times.iter().sum::<u128>();
        let len = millis_times.len() as u128;
        let avg = sum.checked_div(len).expect("Error dividing sum by len");

        aggregation_map.insert(String::from("MIN"), *min);
        aggregation_map.insert(String::from("MAX"), *max);
        aggregation_map.insert(String::from("MEAN"), avg);

        log::info!("action=multiple_api_calls_aggregation, message={:?}", aggregation_map);
        aggregation_map
    }

    fn run_multiple_calls(uri: &String, count_of_calls: u32) -> Vec<u128> {
        let pool = ThreadPool::new(N_WORKERS as usize);
        let (tx, rx) = channel();

        for _ in 0..count_of_calls {
            let tx = tx.clone();
            let uri = uri.clone();
            pool.execute(move || {
                let start = SystemTime::now();
                call_once(&uri);
                let total_time = SystemTime::now().duration_since(start).unwrap();
                tx.send(total_time.as_millis()).expect("Error sending message on tx channel");
            })
        }
        pool.join();
        drop(tx);

        rx.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::metrics::aggregator_timer::aggregator_timer::multiple_api_calls_aggregation;

    #[test]
    fn test_multiple_calls_to_localhost() {
        let uri = String::from("http://localhost");
        let count_of_calls = 100;
        let output = multiple_api_calls_aggregation(&uri, count_of_calls);
        assert!(!output.is_empty());
    }
}