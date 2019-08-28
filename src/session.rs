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
pub struct SessionPackage {
    #[serde(rename = "sessionPtr")]
    pub session_ptr: Qr,
    pub token: String,
}
