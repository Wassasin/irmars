use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SessionStatus {
    Initialized,
    Connected,
    Cancelled,
    Done,
    Timeout,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SessionType {
    Disclosing,
    Signing,
    Issuing,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Qr {
    pub u: String,
    pub irmaqr: SessionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionToken(String);

impl<'a> Into<&'a str> for &'a SessionToken {
    fn into(self) -> &'a str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionPackage {
    #[serde(rename = "sessionPtr")]
    pub session_ptr: Qr,
    pub token: SessionToken,
}

// https://godoc.org/github.com/privacybydesign/irmago/server#SessionResult
#[derive(Serialize, Deserialize, Debug)]
pub struct SessionResult {
    token: SessionToken,
    status: SessionStatus,
    #[serde(rename = "type")]
    stype: SessionType,
    //
}
