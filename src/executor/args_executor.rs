pub mod args_executor {
    use log::info;

    use crate::executor::arg_parser::arg_parser::Arguments;
    use crate::metrics::aggregator_timer::aggregator_timer::multiple_api_calls_aggregation;
    use crate::metrics::basic_timer::basic_timer::single_api_call;

    pub fn main_executor(arguments: &Arguments) {
        match arguments.count {
            Some(count) => {
                info!("action=main_executor, message=Multiple API calls executing");
                let aggregate_duration = multiple_api_calls_aggregation(&arguments.uri, &arguments.method, &arguments.data, &count);
                info!("action=main_executor, calls=multiple, aggregation={:?}", aggregate_duration);
            }
            None => {
                info!("action=main_executor, message=Single API call executing");
                let basic_duration = single_api_call(&arguments.uri, &arguments.method, &arguments.data);
                info!("action=main_executor, calls=single, duration={:?}", basic_duration);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::blocking::Body;

    use crate::executor::arg_parser::arg_parser::Arguments;
    use crate::executor::args_executor::args_executor::main_executor;
    use crate::models::http_methods::methods::Methods;

    #[test]
    fn test_get_call_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: None,
            method: Methods::GET,
            data: None,
        };
        main_executor(&arguments);
    }

    #[test]
    fn test_post_call_valid_uri() {
        let body = Option::Some(Vec::from(Body::from("Hello World").as_bytes().unwrap()));
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: None,
            method: Methods::POST,
            data: body,
        };
        main_executor(&arguments);
    }

    #[test]
    fn test_delete_call_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: None,
            method: Methods::DELETE,
            data: None,
        };
        main_executor(&arguments);
    }

    #[test]
    fn test_put_call_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: None,
            method: Methods::PUT,
            data: None,
        };
        main_executor(&arguments);
    }

    #[test]
    fn test_for_multiple_get_calls_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: Some(300),
            method: Methods::GET,
            data: None,
        };
        main_executor(&arguments);
    }

    #[test]
    fn test_for_multiple_post_calls_valid_uri() {
        let body = Option::Some(Vec::from(Body::from("Hello World").as_bytes().unwrap()));
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: Some(300),
            method: Methods::POST,
            data: body,
        };
        main_executor(&arguments);
    }

    #[test]
    fn test_for_multiple_delete_calls_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: Some(300),
            method: Methods::DELETE,
            data: None,
        };
        main_executor(&arguments);
    }

    #[test]
    fn test_for_multiple_put_calls_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: Some(300),
            method: Methods::PUT,
            data: None,
        };
        main_executor(&arguments);
    }
}