use self::image::Image;
use crate::configs::Configs;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};

mod image;

pub struct AobaService {
    client: Client,
    url: String,
}

impl AobaService {
    pub fn start(configs: &Configs) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();

        if let Some(token) = configs.token.as_ref() {
            let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", token))?;
            auth_value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, auth_value);
        }

        let client = Client::builder().default_headers(headers).build()?;
        Ok(AobaService {
            client,
            url: configs.aoba_url.clone(),
        })
    }

    pub fn format(&self, id: &String) -> String {
        format!("{}/images/{}", self.url, id)
    }

    pub async fn upload(&self, img: &str) -> anyhow::Result<String> {
        let resp = self
            .client
            .post(format!("{}/images/url", self.url))
            .json(&Image {
                url: img.to_owned(),
            })
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }
}
