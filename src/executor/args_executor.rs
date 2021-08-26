pub mod args_executor {
    use crate::executor::arg_parser::arg_parser::Arguments;
    use crate::metrics::aggregator_timer::aggregator_timer::multiple_api_calls_aggregation;
    use crate::metrics::basic_timer::basic_timer::single_api_call;

    pub fn main_executor(arguments: Arguments) {
        match arguments.multiple {
            true => {
                log::info!("action=main_executor, message=Multiple API calls executing");
                multiple_api_calls_aggregation(&arguments.uri, arguments.count);
            }
            false => {
                log::info!("action=main_executor, message=Single API call executing");
                single_api_call(&arguments.uri);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::executor::arg_parser::arg_parser::Arguments;
    use crate::executor::args_executor::args_executor::main_executor;

    #[test]
    fn test_for_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://reddit.com/"),
            count: 0,
            multiple: false,
        };
        main_executor(arguments);
    }

    #[test]
    fn test_for_multiple_calls_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://localhost"),
            count: 300,
            multiple: true,
        };
        main_executor(arguments);
    }
}