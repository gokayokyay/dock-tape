use super::client::{Client, RequestType};
use crate::docker_models::Image;
use serde_json;

impl Client {
    pub async fn list_images(&mut self) -> Vec<Image> {
        // Don't really know if we can do something better ;(
        crate::is_connected!(self);
        let resp = self.request(RequestType::GET, "/images/json").await;
        let images: Vec<Image> = serde_json::from_str(&resp.body).unwrap();
        return images;
    }
}
