pub mod methods {
    #[derive(Debug, Copy, Clone)]
    pub enum Methods {
        GET,
        POST,
        DELETE,
        PUT,
        INVALID,
    }

    pub fn match_string_to_method(method_str: &String) -> Methods {
        if method_str.eq(&String::from("GET")) {
            return Methods::GET;
        } else if method_str.eq(&String::from("POST")) {
            return Methods::POST;
        } else if method_str.eq(&String::from("DELETE")) {
            return Methods::DELETE;
        } else if method_str.eq(&String::from("PUT")) {
            return Methods::PUT;
        }
        Methods::INVALID
    }
}