//! Holochain zome that allow for the registering of profile data on a DID subject
//! Corresponding DID document is expected to contain an extra field; signed_agent which should be signed string of agent who will take ownership over this DID
//! This field is used in validation rules so we know which agent is allowed to edit which profile/DID and if the agent posting the profile actually owns the keys associated to given DID
//!
//! This DNA currently contains a security hole as a result of the inability to do did document resolution for a given incoming did. This is a soft limit of holochain due to its sandboxed execution environment.
//! As a result of this we have no way to tell if incoming DID/Document pair is actually from original DID owner or if the DID subject has been "stolen" and a new document forged.
//! (This pairing of did subject to document is normally protected by a blockchain or trusted entity)
//! Note: a solution for this does exist. Solution involves having another network where a trusted node has some capacities built into its holochain execution sandbox such that it can resolve did's.
//! This network could be bridged and called to validate the integrity of any did subject/document pairs. TODO in the future.

use did_doc::Document;
use hc_utils::WrappedAgentPubKey;
use hdk::prelude::*;
use std::collections::BTreeMap;

mod did_validation;
mod profile;
mod utils;

#[hdk_entry(id = "did", visibility = "public")]
#[derive(Clone)]
pub struct Did(String);

#[hdk_entry(id = "did_document", visibility = "public")]
pub struct DidDocument(Document);

#[hdk_entry(id = "profile", visibility = "public")]
#[derive(Clone)]
pub struct Profile(BTreeMap<String, String>);

entry_defs![
    Did::entry_def(),
    DidDocument::entry_def(),
    Profile::entry_def()
];

/// Get agent information
#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<WrappedAgentPubKey> {
    let agent_info = agent_info()?;

    Ok(WrappedAgentPubKey(agent_info.agent_initial_pubkey))
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct CreateProfileInput {
    pub did: String,
    pub signed_agent: String,
    pub profile: BTreeMap<String, String>,
}

/// Create a profile given DID
#[hdk_extern]
pub fn create_profile(create_data: CreateProfileInput) -> ExternResult<CreateProfileInput> {
    profile::create_profile(create_data.clone())?;
    Ok(create_data)
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct UpdateProfileInput {
    pub did: String,
    pub profile: BTreeMap<String, String>,
}

/// Update profile for a given DID
#[hdk_extern]
pub fn update_profile(update_profile: UpdateProfileInput) -> ExternResult<()> {
    profile::update_profile(update_profile)
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct DidInput(String);

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct GetProfileOutput(Option<BTreeMap<String, String>>);

/// Get the profiles for a given DID
#[hdk_extern]
pub fn get_profile(did: DidInput) -> ExternResult<GetProfileOutput> {
    Ok(GetProfileOutput(profile::get_profile(did)?.map(|p| p.0)))
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct RegisterDidInput {
    pub did: String,
    pub did_document: Document,
}

/// Register a DID in the DHT
#[hdk_extern]
pub fn register_did(register_did: RegisterDidInput) -> ExternResult<()> {
    profile::register_did(register_did)
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct AddProfile {
    pub did: String,
    pub profile: BTreeMap<String, String>,
}

/// Add a profile on already existing DID
#[hdk_extern]
pub fn add_profile(add_profile: AddProfile) -> ExternResult<()> {
    profile::add_profile(add_profile)
}

//Validation logic

//Validate did entry
//Validate did syntax
//Validate integrity of DID

//Validate did document entry
//TODO: resolve did subject and validate that did documents are the same.
//Validate that signed_agent inside did document is the same agent who is trying to post this did document. This is the validation stage that allows for the "claiming/pairing" of a did on this DHT.
//Note that this signed_agent validation doesnt give us anything that isnt already handled by holochain validation logic. It is however useful if we can do did resolving. So we can keep it here ready for the future.
//In the case that we can resolve did's since we can trust a given did subject document pair we can deduce the the agent making the post is the same agent who authored the first claim of this DID on some other system.

//Validate create profile entry
//Validate length/size of entry?
//Perhaps validate that agent does not have more than N profiles already post'd as to reduce possibility of someone spamming network?

//Validate update profile entry
//Validate length/size of entry
//Validate that agent creating update is the same agent who made the first profile entry
//Actually possible here that we could allow multiple agents to update profile entry if the did document had multiple signed_agent fields where each signed_agent was allowed editable agent
//Editing from multiple agents would require that profile has links to did document so that we can check this signed agents field

//Validate links

//did subject -> did document:
//Validate that author of subject and document are the same. Since creating a did document entry requires the validation of signed_agent field we can be sure that author of did document is the rightful owner of this did.
//Validate that subject inside did document is the same as the did subject as source for this link.

//did subject -> profile
//Validate that there is a link between did subject -> did document. This gives us the verification that creator of did subject is same agent as creator of did document.
//Validate that agent posting profile is the same agent who created the did subject.
