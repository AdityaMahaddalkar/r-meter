pub mod arg_parser {
    use clap::{App, Arg, ArgMatches};
    use http::Uri;
    use log::{
        error, info,
    };

    #[derive(Debug)]
    pub struct Arguments {
        pub(crate) uri: String,
    }

    impl Arguments {
        pub fn get_uri(&self) -> &String {
            &self.uri
        }
    }

    pub fn parse_args() -> Arguments {
        let matches = App::new("R-Meter")
            .version("0.0.1")
            .author("Aditya <adityam1311@gmail.com>")
            .about("Rust-based executable for API metrics")
            .arg(Arg::with_name("uri")
                .short("u")
                .long("uri")
                .takes_value(true)
                .help("A valid URI of API endpoint")
            )
            .get_matches();


        is_uri_valid(&matches);

        let uri = String::from(matches.value_of("uri").expect("Invalid URI passed"));

        Arguments {
            uri
        }
    }

    fn is_uri_valid(matches: &ArgMatches) {
        let uri_string = match matches.value_of("uri") {
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
}