const HTTP_METHOD: &'static str = "method";
const URI_FLAG: &'static str = "uri";
const DATA: &'static str = "data";
const FILE: &'static str = "file";
const COUNT_FLAG: &'static str = "count";


pub mod arg_parser {
    use std::fs::File;

    use clap::{App, Arg, ArgMatches};
    use http::Uri;
    use log::{
        error, info,
    };
    use reqwest::blocking::Body;

    use crate::executor::arg_parser::{COUNT_FLAG, DATA, FILE, HTTP_METHOD, URI_FLAG};
    use crate::models::http_methods::methods::{match_string_to_method, Methods};

    #[derive(Debug)]
    pub struct Arguments {
        pub(crate) uri: String,
        pub(crate) count: Option<u32>,
        pub(crate) method: Methods,
        pub(crate) data: Option<Vec<u8>>,
    }

    pub fn parse_args() -> Arguments {
        let matches = App::new("R-Meter")
            .version("0.0.1")
            .author("Aditya <adityam1311@gmail.com>")
            .about("Rust-based executable for API metrics")
            .arg(Arg::with_name(HTTP_METHOD)
                .short("m")
                .long("method")
                .takes_value(true)
                .required(true)
                .possible_values(&["GET", "POST", "DELETE", "PUT"])
                .help("HTTP method name to execute on the URI"))
            .arg(Arg::with_name(URI_FLAG)
                .short("u")
                .long("uri")
                .takes_value(true)
                .help("A valid URI of API endpoint")
            )
            .arg(Arg::with_name(DATA)
                .short("d")
                .long("data")
                .takes_value(true)
                .required(false)
                .help("(Optional) JSON data for request body, if any"))
            .arg(Arg::with_name(FILE)
                .short("f")
                .long("file")
                .takes_value(true)
                .required(false)
                .help("(Optional) File path for fetching data for request body, if any"))
            .arg(Arg::with_name(COUNT_FLAG)
                .short("c")
                .long("count")
                .required(false)
                .takes_value(true)
                .help("Count of requests to be sent"))
            .get_matches();


        is_http_method_valid(&matches);
        is_uri_valid(&matches);

        let uri = String::from(matches.value_of(URI_FLAG).expect("Invalid URI passed"));
        let method = match_string_to_method(&String::from(matches.value_of(HTTP_METHOD)
            .expect("Invalid HTTP method passed")));
        let count = is_multiple_set_with_valid_count(&matches);
        let data = get_data_from_arguments(&matches);

        Arguments {
            uri,
            count,
            method,
            data,
        }
    }

    fn is_http_method_valid(matches: &ArgMatches) {
        let method_string = matches.value_of(HTTP_METHOD)
            .expect("No method string passed in arguments");
        let _ = match match_string_to_method(&String::from(method_string)) {
            Methods::GET => info!("action=is_http_method_valid, httpMethod={:?}", Methods::GET),
            Methods::POST => info!("action=is_http_method_valid, httpMethod={:?}", Methods::GET),
            Methods::DELETE => info!("action=is_http_method_valid, httpMethod={:?}", Methods::GET),
            Methods::PUT => info!("action=is_http_method_valid, httpMethod={:?}", Methods::GET),
            Methods::INVALID => {
                error!("action=is_http_method_valid, invalid HTTP Method, please enter a valid method");
                panic!()
            }
        };
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

    fn get_data_from_arguments(matches: &ArgMatches) -> Option<Vec<u8>> {
        if matches.is_present(DATA) {
            return match matches.value_of(DATA) {
                None => None,
                Some(data) => {
                    let data = String::from(data);
                    let body_bytes = Vec::from(data.as_bytes());
                    return Some(body_bytes);
                }
            };
        } else if matches.is_present(FILE) {
            return match matches.value_of(FILE) {
                None => None,
                Some(file_path) => {
                    let file = File::open(file_path).unwrap();
                    let body_bytes = Vec::from(Body::from(file).as_bytes().unwrap());
                    return Some(body_bytes);
                }
            };
        }
        None
    }

    fn is_multiple_set_with_valid_count(matches: &ArgMatches) -> Option<u32> {
        if matches.is_present(COUNT_FLAG) {
            return match matches.value_of(COUNT_FLAG) {
                None => {
                    error!("action=is_multiple_set_with_valid_count, message=Value of count flag not set. Setting it to default 100");
                    None
                }
                Some(value) => {
                    let ret_count = value.parse::<u32>().unwrap();
                    Some(ret_count)
                }
            };
        }
        None
    }
}