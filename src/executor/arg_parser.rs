const URI_FLAG: &'static str = "uri";
const MULTIPLE_FLAG: &'static str = "multiple";
const COUNT_FLAG: &'static str = "count";


pub mod arg_parser {
    use clap::{App, Arg, ArgMatches};
    use http::Uri;
    use log::{
        error, info,
    };

    use crate::executor::arg_parser::{COUNT_FLAG, MULTIPLE_FLAG, URI_FLAG};

    #[derive(Debug)]
    pub struct Arguments {
        pub(crate) uri: String,
        pub(crate) multiple: bool,
        pub(crate) count: u32,
    }

    pub fn parse_args() -> Arguments {
        let matches = App::new("R-Meter")
            .version("0.0.1")
            .author("Aditya <adityam1311@gmail.com>")
            .about("Rust-based executable for API metrics")
            .arg(Arg::with_name(URI_FLAG)
                .short("u")
                .long("uri")
                .takes_value(true)
                .help("A valid URI of API endpoint")
            )
            .arg(Arg::with_name(MULTIPLE_FLAG)
                .short("m")
                .long("multi")
                .required(false)
                .takes_value(false)
                .help("Boolean flag for multiple calls"))
            .arg(Arg::with_name(COUNT_FLAG)
                .short("c")
                .long("count")
                .required(false)
                .takes_value(true)
                .help("Count of requests to be sent"))
            .get_matches();


        is_uri_valid(&matches);

        let uri = String::from(matches.value_of(URI_FLAG).expect("Invalid URI passed"));

        let (multiple, count) = is_multiple_set_with_valid_count(&matches);

        Arguments {
            uri,
            multiple,
            count,
        }
    }

    fn is_uri_valid(matches: &ArgMatches) {
        let uri_string = match matches.value_of(URI_FLAG) {
            None => {
                error!("action=is_uri_valid, message=No URI mentioned in command line arguments!");
                panic!()
            }
            Some(uri) => {
                uri
            }
        };

        let _ = match uri_string.parse::<Uri>() {
            Ok(uri) => {
                info!("action=is_uri_valid, message=Valid URI: {:?} mentioned", uri)
            }
            Err(_) => {
                error!("action=is_uri_valid, message=Cannot parse entered URI, please check the format of URI");
                panic!()
            }
        };
    }

    fn is_multiple_set_with_valid_count(matches: &ArgMatches) -> (bool, u32) {
        let mut ret_multiple = false;
        let mut ret_count: u32 = 0;
        if matches.is_present(MULTIPLE_FLAG) {
            let count_flag: u32 = match matches.value_of(COUNT_FLAG) {
                None => {
                    error!("action=is_multiple_set_with_valid_count, message=Value of count flag not set. Setting it to default 100");
                    100
                }
                Some(value) => {
                    value.parse::<u32>().unwrap()
                }
            };
            ret_count = count_flag;
            ret_multiple = true;
        }
        (ret_multiple, ret_count)
    }
}