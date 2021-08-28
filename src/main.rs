use crate::executor::arg_parser::arg_parser::parse_args;
use crate::executor::args_executor::args_executor::main_executor;

mod logging;
mod metrics;
mod executor;
mod models;


fn main() {
    logging::log_initializer::log_init();

    let arguments = parse_args();
    main_executor(&arguments);
}

