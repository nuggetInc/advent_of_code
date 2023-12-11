use std::env;

use reqwest::{
    blocking::{Client, Response},
    Error, IntoUrl,
};

pub struct AocClient {
    client: Client,
    session: String,
}

impl AocClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            session: env::var("AOC_SESSION").expect("AOC_SESSION was not set"),
        }
    }

    pub fn get(&self, url: impl IntoUrl) -> Result<Response, Error> {
        let cookie = format!("session={}", self.session);
        self.client.get(url).header("Cookie", cookie).send()
    }
}
