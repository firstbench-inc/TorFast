use std::collections::HashMap;

pub struct Poster {
    client: reqwest::Client,
}

impl Poster {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Poster { client }
    }

    pub async fn post_url_data(
        &self,
        data: &HashMap<String, String>,
    ) -> Result<(), reqwest::Error> {
        let res = match self
            .client
            .post("localhost:9200/logs/_doc")
            .json(data)
            .send()
            .await {
                Ok(res) => res,
                Err(e) => {
                    return Err(e);
                }
            };

        println!("Posted data: {:?}", res);
        Ok(())
    }
}
