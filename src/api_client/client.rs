pub use super::models::{Client, RequestType, Response};
use std::{io::prelude::*, io::Error, os::unix::net::UnixStream};

impl Client {
    pub fn new(socket_addr: &str, api_version: &str) -> Self {
        let client = Client {
            socket_addr: socket_addr.to_string(),
            api_version: api_version.to_string(),
            stream: None,
        };
        client
    }
    pub fn default() -> Self {
        let mut client = Client::new("/var/run/docker.sock", "/v1.40");
        let addr = client.socket_addr.clone();
        client.connect(addr).unwrap();
        return client;
    }
    pub fn connect(&mut self, socket_addr: String) -> Result<(), Error> {
        let stream = UnixStream::connect(socket_addr);
        self.stream = Some(stream?);
        Ok(())
    }
    pub fn construct_request(request_type: RequestType, version: &str, path: &str) -> String {
        let mut result: String = String::new();
        let suffix = " HTTP/1.0\r\n\r\n";
        result.push_str(&request_type.to_string());
        result.push_str(&(version.to_string() + path));
        result.push_str(suffix);
        result.to_string()
    }
    pub async fn request(&mut self, request_type: RequestType, path: &str) -> Response {
        let req = Self::construct_request(request_type, &self.api_version, path);
        let mut stream = match self.stream.as_ref() {
            Some(x) => x,
            _ => panic!("Client is not connected! Please check your user group, check socket address and user group.")
        };
        stream.write_all(req.as_bytes()).unwrap();
        let mut resp = String::new();
        stream.read_to_string(&mut resp).unwrap();

        self.parse_response(resp)
    }
    fn parse_response(&self, response: String) -> Response {
        let mut headers_raw: Vec<u8> = Vec::new();
        let mut body_raw: Vec<u8> = Vec::new();
        let mut body_index: usize = 0;
        let raw = response.as_bytes();
        // headers
        for i in 0..raw.len() {
            if i + 3 >= raw.len() {
                break;
            }

            // CRLFCRLF
            if raw[i] == 13 && raw[i + 1] == 10 && raw[i + 2] == 13 && raw[i + 3] == 10 {
                body_index = i + 4;
                if body_index >= raw.len() {
                    body_index = raw.len() - 1;
                }
                break;
            }

            headers_raw.push(raw[i]);
        }

        // body
        for i in body_index..raw.len() {
            body_raw.push(raw[i]);
        }
        let headers = std::str::from_utf8(&headers_raw).unwrap();
        let body = std::str::from_utf8(&body_raw).unwrap();

        Response {
            headers: headers.to_string(),
            body: body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api_client::models::{Client, RequestType};
    use crate::docker_models::Image;
    use serde_json;
    use tokio::runtime::Runtime;
    #[tokio::main]
    #[test]
    #[should_panic]
    async fn client_is_not_connected() {
        // let mut client = Client { socket_addr: "".to_string(), stream: None };
        let mut client = Client::new("/var/run/docker.sock-fail", "/v1.40");
        let _ = client.request(RequestType::GET, "/images/json").await;
    }
    #[test]
    fn client_is_connected() {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut client = Client::default();
            client.connect("/var/run/docker.sock".to_string()).unwrap();
            let a = client.request(RequestType::GET, "/images/json").await;
            // println!("{:?}", a.body);
            let _b: Vec<Image> = serde_json::from_str(&a.body).unwrap();
            // println!("{:?}", b);
        });
    }
    #[tokio::main]
    #[test]
    async fn return_images_connected() {
        let mut client = Client::default();
        let a = client.list_images().await;
        println!("{:?}", a);
    }
    #[tokio::main]
    #[test]
    #[should_panic]
    async fn return_images_not_connected() {
        let mut client = Client::new("/var/run/docker.sock-fail", "/v1.40");
        let _a = client.list_images().await;
    }
}
