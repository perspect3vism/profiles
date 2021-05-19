use chrono::{DateTime, Utc};
use std::collections::BTreeMap;
use hdk::prelude::*;
use did_doc::Document;

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct DidInput(pub String);

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct RegisterDidInput {
    pub did: String,
    pub did_document: Document,
}


#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ProfileInput {
    pub author: Agent,
    pub data: ExpressionData,
    pub proof: ExpressionProof,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct Agent {
    pub did: String,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct ExpressionData {
    pub signed_agent: String,
    pub profile: BTreeMap<String, String>,
    #[serde(rename(serialize = "@context", deserialize = "@context"))]
    pub context: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}