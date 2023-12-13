use std::{collections::HashMap, env};

use reqwest::{
    blocking::{Client, Response},
    Error, IntoUrl,
};

pub struct AocClient {
    client: Client,
    session: String,
}

impl AocClient {
    pub fn get(&self, url: impl IntoUrl) -> Result<Response, Error> {
        let cookie = format!("session={}", self.session);
        self.client.get(url).header("Cookie", cookie).send()
    }

    pub fn post(
        &self,
        url: impl IntoUrl,
        params: HashMap<String, String>,
    ) -> Result<Response, Error> {
        let cookie = format!("session={}", self.session);
        self.client
            .post(url)
            .header("Cookie", cookie)
            .form(&params)
            .send()
    }
}

impl Default for AocClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            session: env::var("AOC_SESSION").expect("AOC_SESSION is not set"),
        }
    }
}
