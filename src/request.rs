use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::BTreeMap;

/// An AttributeRequest asks for an instance of an attribute type,
/// possibly requiring it to have a specified value, in a session request.
#[derive(Serialize, Deserialize)]
pub struct AttributeRequest {
    #[serde(rename = "type")]
    pub atype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub not_null: bool,
}

/// A conjuction of attribute requests, only satisfied
/// when all of its containing attribute requests are satisfied.
#[derive(Serialize, Deserialize)]
pub struct AttributeCon(pub Vec<AttributeRequest>);

/// A disjunction of conjunction of attribute requests, only satisfied
/// when at least one of its containing attribute request conjunctions is satisfied.
#[derive(Serialize, Deserialize)]
pub struct AttributeDisCon(pub Vec<AttributeCon>);

/// AttributeConDisCon is only satisfied if all of the containing AttributeDisCon are satisfied.
#[derive(Serialize, Deserialize)]
pub struct AttributeConDisCon(pub Vec<AttributeDisCon>);

/// A DisclosureRequest is a request to disclose certain attributes.
#[derive(Deserialize)]
pub struct DisclosureRequest {
    pub disclose: AttributeConDisCon,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<usize, BTreeMap<String, String>>>,
}

impl Serialize for DisclosureRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = 2 + if self.labels.is_some() { 1 } else { 0 };
        let mut dr = serializer.serialize_struct("DisclosureRequest", len)?;
        dr.serialize_field("@context", "https://irma.app/ld/request/disclosure/v2")?;
        dr.serialize_field("disclose", &self.disclose)?;

        if self.labels.is_some() {
            dr.serialize_field("labels", &self.labels)?;
        }

        dr.end()
    }
}
