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
    // Couldn't manage to work it out?!
    // pub async fn build_image(&mut self) -> Result<(), std::io::Error> {
    //     crate::is_connected!(self);
    //     let a = fs::read_to_string("Dockerfile")?;
    //     println!("{}", &a);
    //     let mut req = Self::construct_request(RequestType::POST, &self.api_version, "/build");
    //     req.push_str(&a);
    //     println!("{}", &req);
    //     let resp = self
    //         .request_with_str(&req)
    //         .await;
    //     println!("{:?}", resp);
    //     Ok(())
    // }
}
