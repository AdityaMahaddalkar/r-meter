pub mod client {
    use log::error;

    use crate::models::http_methods::methods::Methods;

    pub fn call_uri_with_method(uri: &String, method: &Methods, body: &Option<Vec<u8>>) {
        let client = reqwest::blocking::Client::new();
        let body = body.clone();
        match method {
            Methods::GET => { client.get(uri).send().expect("Error executing GET call on URI"); }
            Methods::POST => {
                match body {
                    None => { client.post(uri).send().expect("Error executing POST call on URI"); }
                    Some(data) => { client.post(uri).body(data).send().expect("Error executing POST call on URI"); }
                }
            }
            Methods::DELETE => { client.delete(uri).send().expect("Error executing DELETE call on URI"); }
            Methods::PUT => {
                match body {
                    None => { client.put(uri).send().expect("Error executing PUT call on URI"); }
                    Some(data) => { client.put(uri).body(data).send().expect("Error executing PUT call on URI"); }
                }
            }
            Methods::INVALID => {
                error!("action=call_once, message=Invalid HTTP method set, please verify the method correctly");
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use reqwest::blocking::Body;

    use crate::metrics::client::client::call_uri_with_method;
    use crate::models::http_methods::methods::Methods;

    #[test]
    fn test_call_get_once() {
        let valid_uri = String::from("http://reddit.com/");
        call_uri_with_method(&valid_uri, &Methods::GET, &None);
    }

    #[test]
    fn test_call_post_once() {
        let body = Vec::from(Body::from("Hello world").as_bytes().unwrap());
        let valid_uri = String::from("http://reddit.com/");
        call_uri_with_method(&valid_uri, &Methods::POST, &Some(body));
    }

    #[test]
    fn test_call_delete_once() {
        let valid_uri = String::from("http://reddit.com/");
        call_uri_with_method(&valid_uri, &Methods::DELETE, &None);
    }

    #[test]
    fn test_call_put_once() {
        let valid_uri = String::from("http://reddit.com/");
        call_uri_with_method(&valid_uri, &Methods::PUT, &None);
    }
}