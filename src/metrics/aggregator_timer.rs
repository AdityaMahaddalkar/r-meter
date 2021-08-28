const N_WORKERS: i32 = 8;

pub mod aggregator_timer {
    use std::sync::mpsc::channel;
    use std::time::{Duration, SystemTime};

    use log::info;
    use threadpool::ThreadPool;

    use crate::metrics::aggregator_timer::N_WORKERS;
    use crate::metrics::client::client::call_uri_with_method;
    use crate::models::duration_output::AggregateDuration;
    use crate::models::http_methods::methods::Methods;

    pub fn multiple_api_calls_aggregation(uri: &String, methods: &Methods, body: &Option<Vec<u8>>, count_of_calls: &u32) -> AggregateDuration {
        info!("action=multiple_api_calls_aggregation, method={:?}, uri={}", methods, uri);
        let millis_times = run_multiple_calls(uri, methods, body, count_of_calls);

        let min = millis_times.iter().min().unwrap();
        let max = millis_times.iter().max().unwrap();
        let sum = millis_times.iter().sum::<u128>();
        let len = millis_times.len() as u128;
        let avg = sum.checked_div(len).expect("Error dividing sum by len");

        AggregateDuration {
            min_time_to_execute: Duration::from_millis(*min as u64),
            max_time_to_execute: Duration::from_millis(*max as u64),
            mean_time_to_execute: Duration::from_millis(avg as u64),
        }
    }

    fn run_multiple_calls(uri: &String, methods: &Methods, body: &Option<Vec<u8>>, count_of_calls: &u32) -> Vec<u128> {
        let pool = ThreadPool::new(N_WORKERS as usize);
        let (tx, rx) = channel();

        for _ in 0..*count_of_calls {
            let tx = tx.clone();
            let uri = uri.clone();
            let method = methods.clone();
            let body = body.clone();
            pool.execute(move || {
                let start = SystemTime::now();
                call_uri_with_method(&uri, &method, &body);
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
    use reqwest::blocking::Body;

    use crate::metrics::aggregator_timer::aggregator_timer::multiple_api_calls_aggregation;
    use crate::models::http_methods::methods::Methods;

    #[test]
    fn test_multiple_get_calls() {
        let uri = String::from("http://localhost");
        let method = Methods::GET;
        let count_of_calls = 100;
        let _ = multiple_api_calls_aggregation(&uri, &method, &None, &count_of_calls);
    }

    #[test]
    fn test_multiple_post_calls() {
        let uri = String::from("http://localhost");
        let method = Methods::POST;
        let count_of_calls = 100;
        let body = Vec::from(Body::from("Hello World").as_bytes().unwrap());
        let _ = multiple_api_calls_aggregation(&uri, &method, &Some(body), &count_of_calls);
    }

    #[test]
    fn test_multiple_delete_calls() {
        let uri = String::from("http://localhost");
        let method = Methods::DELETE;
        let count_of_calls = 100;
        let _ = multiple_api_calls_aggregation(&uri, &method, &None, &count_of_calls);
    }

    #[test]
    fn test_multiple_put_calls() {
        let uri = String::from("http://localhost");
        let method = Methods::PUT;
        let count_of_calls = 100;
        let _ = multiple_api_calls_aggregation(&uri, &method, &None, &count_of_calls);
    }
}