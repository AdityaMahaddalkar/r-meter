pub mod client {
    pub fn call_once(uri: &String) {
        match reqwest::blocking::get(uri) {
            Ok(successful_response) => {
                log::info!("{:?}", successful_response);
            }
            Err(_) => {
                log::error!("action=call_once, message=Failed to GET URI={}", uri);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::metrics::client::client::call_once;

    #[test]
    fn test_call_once() {
        let valid_uri = String::from("http://reddit.com/");
        call_once(&valid_uri);
    }
}