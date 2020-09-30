use futures::future::Future;
use reqwest::r#async::ClientBuilder;
use reqwest::Url;

use crate::request::*;
use crate::session::*;

pub struct Client {
    baseurl: String,
    client: reqwest::r#async::Client,
}

pub type Error = reqwest::Error;
pub type StatusCode = reqwest::StatusCode;

impl Client {
    pub fn new(baseurl: String) -> Result<Client, Error> {
        let client = ClientBuilder::new().build().unwrap();

        Ok(Client { baseurl, client })
    }

    fn create_url(&self, u: &str) -> Url {
        Url::parse(&self.baseurl).unwrap().join(u).unwrap()
    }

    pub fn request(
        &self,
        dr: &DisclosureRequest,
    ) -> impl Future<Item = SessionPackage, Error = Error> {
        self.client
            .post(self.create_url("session"))
            .json(dr)
            .send()
            .and_then(|resp| resp.error_for_status())
            .and_then(|mut resp| resp.json::<SessionPackage>())
    }

    pub fn cancel(&self, token: &SessionToken) -> impl Future<Item = (), Error = Error> {
        self.client
            .delete(self.create_url("session/").join(token.into()).unwrap())
            .send()
            .and_then(|resp| resp.error_for_status())
            .and_then(|resp| {
                resp.error_for_status()?;
                Ok(())
            })
    }

    pub fn result(&self, token: &SessionToken) -> impl Future<Item = SessionResult, Error = Error> {
        let token: &str = token.into();
        self.client
            .get(
                self.create_url("session/")
                    .join(&format!("{}/result", token))
                    .unwrap(),
            )
            .send()
            .and_then(|resp| resp.error_for_status())
            .and_then(|mut resp| resp.json::<SessionResult>())
    }
}
