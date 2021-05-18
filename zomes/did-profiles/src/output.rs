use hdk::prelude::*;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextProfileResponse {
    #[serde(rename(serialize = "@context", deserialize = "@context"))]
    pub context: BTreeMap<String, String>,
    #[serde(flatten)]
    pub profile_data: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct GetProfileResponse(pub Option<ContextProfileResponse>);
