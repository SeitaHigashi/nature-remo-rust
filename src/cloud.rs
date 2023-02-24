pub mod object;
pub mod get;
pub mod post;
pub mod api;

use self::get::GET;
use self::api::API;

use reqwest;
use url::Url;

pub struct Client {
    token: String,
    base_url: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new(token: String) -> Self {
        Client {
            token, 
            base_url: Url::parse("https://api.nature.global").unwrap(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, api: GET) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.base_url.join(&api.path()).unwrap();
        let res = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await;
        res
    }

}
