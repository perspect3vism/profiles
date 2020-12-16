//! Holochain zome that allow for the registering of profile data on some DID item
//! Note that the DID document must contain an extra field; signed_agent which should be signed string of agent who will take ownership over this DID
//! This field is used in validation rules so we know which agent is allowed to edit which profile/DID and if the agent posting the profile actually owns the keys associated to given DID

use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::*;
use did_doc::Document;
use std::collections::BTreeMap;

mod profile;

#[hdk_entry(id = "did", visibility = "public")]
#[derive(Clone)]
pub struct Did(String);

#[hdk_entry(id = "did_document", visibility = "public")]
pub struct DidDocument(Document);

#[hdk_entry(id = "profile", visibility = "public")]
#[derive(Clone)]
pub struct Profile(BTreeMap<String, String>);

entry_defs![Did::entry_def(), DidDocument::entry_def(), Profile::entry_def()];

/** Profiles **/

///Get agent information
#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info()?;

    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct CreateProfileInput {
    pub did: String,
    pub did_document: Document,
    pub profile: BTreeMap<String, String>
}

/// Create a profile given DID
#[hdk_extern]
pub fn create_profile(create_data: CreateProfileInput) -> ExternResult<()> {
    profile::create_profile(create_data)
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct UpdateProfileInput {
    pub did: String,
    pub profile: BTreeMap<String, String>
}

/// Update profile for a given DID
#[hdk_extern]
pub fn update_profile(update_profile: UpdateProfileInput) -> ExternResult<()> {
    Ok(())
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct DidInput(String);

/// Get the profiles for a given DID
#[hdk_extern]
pub fn get_profiles(did: DidInput) -> ExternResult<()> {
    Ok(())
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct RegisterDidInput {
    pub did: String,
    pub did_document: Document,
}

/// Register a DID in the DHT
#[hdk_extern]
pub fn register_did(register_did: RegisterDidInput) -> ExternResult<()> {
    Ok(())
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct AddProfile {
    pub did: String,
    pub profile: BTreeMap<String, String>
}

/// Add a profile on already existing DID
#[hdk_extern]
pub fn add_profile(add_profile: AddProfile) -> ExternResult<()> {
    Ok(())
}