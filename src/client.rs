use super::*;
use futures::future::Future;
use reqwest::r#async::ClientBuilder;
use reqwest::Url;

pub struct Client {
    baseurl: String,
    client: reqwest::r#async::Client,
}

pub type ClientError = reqwest::Error;

impl Client {
    pub fn new(baseurl: String) -> Result<Client, reqwest::Error> {
        let client = ClientBuilder::new().build().unwrap();

        Ok(Client { baseurl, client })
    }

    fn create_url(&self, u: &str) -> Url {
        Url::parse(&self.baseurl).unwrap().join(u).unwrap()
    }

    pub fn request(
        &self,
        dr: &DisclosureRequest,
    ) -> impl Future<Item = SessionPackage, Error = ClientError> {
        self.client
            .post(self.create_url("/session"))
            .json(dr)
            .send()
            .and_then(|mut resp| resp.json::<SessionPackage>())
    }

    pub fn cancel(&self, token: &SessionToken) -> impl Future<Item = (), Error = ClientError> {
        self.client
            .delete(self.create_url("/session/").join(token.into()).unwrap())
            .send()
            .and_then(|resp| {
                resp.error_for_status()?;
                Ok(())
            })
    }
}
