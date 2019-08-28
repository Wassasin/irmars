use super::*;
use futures::future::Future;
use reqwest::r#async::ClientBuilder;
use reqwest::Url;

pub struct Client {
    baseurl: String,
    client: reqwest::r#async::Client,
}

impl Client {
    pub fn new(baseurl: String) -> Result<Client, reqwest::Error> {
        let client = ClientBuilder::new().build().unwrap();

        Ok(Client { baseurl, client })
    }

    pub fn request(
        &self,
        dr: &DisclosureRequest,
    ) -> impl Future<Item = SessionPackage, Error = reqwest::Error> {
        self.client
            .post(Url::parse(&format!("{}/session", self.baseurl)).unwrap())
            .json(dr)
            .send()
            .and_then(|mut resp| resp.json::<SessionPackage>())
    }
}
