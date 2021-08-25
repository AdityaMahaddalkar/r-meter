pub mod args_executor {
    use std::time::Duration;

    use log::info;

    use crate::executor::arg_parser::arg_parser::Arguments;
    use crate::metrics::basic_timer::basic_timer::single_api_call;

    pub fn main_executor(arguments: Arguments) -> Duration {
        info!("action=main_executor, arguments={:?}", arguments);
        single_api_call(arguments.get_uri())
    }
}

#[cfg(test)]
mod tests {
    use crate::executor::arg_parser::arg_parser::Arguments;
    use crate::executor::args_executor::args_executor::main_executor;

    #[test]
    fn test_for_valid_uri() {
        let arguments = Arguments {
            uri: String::from("http://reddit.com/")
        };
        let not_expected: u128 = 0;
        let actual = main_executor(arguments).as_nanos();
        assert_ne!(not_expected, actual);
    }
}