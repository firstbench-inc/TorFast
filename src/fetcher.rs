use std::fmt::Debug;

use reqwest::{Client, IntoUrl, Proxy};

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    // Constructor to create a new Fetcher instance
    pub fn new() -> Self {
        let proxy = Proxy::all("http://127.0.0.1:9050")
            .expect("tor proxy should be there");
        let client = Client::builder()
            .proxy(proxy)
            .build()
            .expect("should be able to build reqwest client");

        Fetcher { client }
    }

    // The fetch method
    pub async fn fetch<S: Into<String> + IntoUrl + Clone + Debug>(
        &self,
        url: S,
    ) -> Result<String, reqwest::Error> {
        let url_str = url.clone();
        let res = self.client.get(url).send().await?;
        println!("Status: {} URL {:?}", res.status(),url_str);

        let text = res.text().await?;
        Ok(text)
    }
}
