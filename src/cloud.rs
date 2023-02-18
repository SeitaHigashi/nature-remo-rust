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
            token: token, f
            base_url: Url::parse("https://api.nature.global").unwrap(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.base_url.join("/1/appliances").unwrap();
        let res = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await;
        println!("{:?}", res);
        res
    }

}
