use std::os::unix::net::UnixStream;

#[derive(Debug)]
pub struct Client {
    pub socket_addr: String,
    pub stream: Option<UnixStream>,
    pub api_version: String, // TODO: Authentication
}
#[derive(Debug)]
pub struct Response {
    pub headers: String,
    pub body: String,
}

pub enum RequestType {
    GET,
    PUT,
    POST,
    DELETE,
}

impl ToString for RequestType {
    fn to_string(&self) -> std::string::String {
        match self {
            Self::DELETE => "GET ".to_string(),
            Self::PUT => "PUT ".to_string(),
            Self::POST => "POST ".to_string(),
            Self::GET => "GET ".to_string(),
        }
    }
}
