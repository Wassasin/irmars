use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionStatus {
    Initialized,
    Connected,
    Cancelled,
    Done,
    Timeout,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProofStatus {
    /// Proof is valid
    Valid,
    /// Proof is invalid
    Invalid,
    /// Attribute-based signature had invalid timestamp
    InvalidTimestamp,
    /// Proof does not correspond to a specified request
    UnmatchedRequest,
    /// Proof does not contain all requested attributes
    MissingAttributes,
    /// Attributes were expired at proof creation time (now, or according to timestamp in case of abs)
    Expired,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttributeProofStatus {
    /// Attribute is disclosed and matches the value
    Present,
    /// Attribute is disclosed, but wasn't requested in request
    Extra,
    /// Attribute is disclosed but is null
    Null,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
pub struct SessionToken(pub String);

impl<'a> Into<&'a str> for &'a SessionToken {
    fn into(self) -> &'a str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SessionPackage {
    pub session_ptr: Qr,
    pub token: SessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisclosedAttribute {
    pub rawvalue: Option<String>,
    pub value: BTreeMap<String, String>,
    pub id: String,
    pub status: AttributeProofStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SessionResult {
    pub token: SessionToken,
    pub status: SessionStatus,
    #[serde(rename = "type")]
    pub stype: SessionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_status: Option<ProofStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosed: Option<Vec<Vec<DisclosedAttribute>>>,
    // TODO signature irma.SignedMessage
    // TODO error irma.RemoteError
}
