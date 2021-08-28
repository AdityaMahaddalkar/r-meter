pub mod basic_timer {
    use std::time::SystemTime;

    use log::info;

    use crate::metrics::client::client::call_uri_with_method;
    use crate::models::duration_output::BasicDuration;
    use crate::models::http_methods::methods::Methods;

    pub fn single_api_call(uri: &String, methods: &Methods, body: &Option<Vec<u8>>) -> BasicDuration {
        let start = SystemTime::now();
        info!("action=single_api_call, method={:?}, uri={}", methods, uri);
        call_uri_with_method(uri, &methods, &body);
        let total_execution_time = SystemTime::now().duration_since(start).expect("Error in system time");
        info!("action=single_api_call, duration={:?}", total_execution_time);

        BasicDuration {
            time_to_execute: total_execution_time
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::blocking::Body;

    use crate::metrics::basic_timer::basic_timer::single_api_call;
    use crate::models::http_methods::methods::Methods;

    const URI_TO_QUERY: &'static str = "http://reddit.com";

    #[test]
    fn test_single_api_get_call_valid_uri() {
        let valid_uri = String::from(URI_TO_QUERY);
        single_api_call(&valid_uri, &Methods::GET, &None);
    }

    #[test]
    fn test_single_api_post_call_valid_uri() {
        let body = Body::from("Hello World");
        let valid_uri = String::from(URI_TO_QUERY);
        single_api_call(&valid_uri, &Methods::POST, &Some(Vec::from(body.as_bytes().unwrap())));
    }

    #[test]
    fn test_single_api_delete_call_valid_uri() {
        let valid_uri = String::from(URI_TO_QUERY);
        single_api_call(&valid_uri, &Methods::DELETE, &None);
    }

    #[test]
    fn test_single_api_put_call_valid_uri() {
        let valid_uri = String::from(URI_TO_QUERY);
        single_api_call(&valid_uri, &Methods::PUT, &None);
    }
}