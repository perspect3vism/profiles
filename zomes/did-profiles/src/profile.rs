use did_doc::Uri;
use hc_utils::{get_header, get_latest_entry, get_latest_link};
use hdk::prelude::*;
use std::str::FromStr;

use crate::did_validation::validate_did_doc;
use crate::utils::{did_validate_and_check_integrity, err, try_from_entry};
use crate::{
    AddProfile, CreateProfileInput, Did, DidDocument, DidInput, Profile, RegisterDidInput,
    UpdateProfileInput,
};

pub fn create_profile(create_data: CreateProfileInput) -> ExternResult<()> {
    //Validate did
    let (did, did_hash) = did_validate_and_check_integrity(&create_data.did, false)?;

    //Resolve did document from trusted did resolver DHT
    //Validate resolved did document and that the signed_agent field received by this function was signed by key in did
    //validate_did_doc(&create_data.did_document)?;

    //Add signed_agent field to did document

    //Add document entry; we commit this first so the did document can be used as the proof of did ownership for this agent
    // let did_doc = DidDocument(create_data.did_document);
    // let did_doc_hash = hash_entry(&did_doc)?;
    // create_entry(&did_doc)?;

    //We would need some way to find above did document in did subjects entry validation so we would also likely need a link here
    //From some temp_did_subject anchor -> did_document

    //Then add the did entry
    create_entry(&did)?;

    //Link document entry to did
    // create_link(
    //     did_hash.clone(),
    //     did_doc_hash,
    //     LinkTag::from("doc".as_bytes().to_owned()),
    // )?;

    //Add profile entry
    let did_profile = Profile(create_data.context, create_data.profile);
    let did_profile_hash = hash_entry(&did_profile)?;
    create_entry(&did_profile)?;

    //Link profile entry to did
    create_link(
        did_hash,
        did_profile_hash,
        LinkTag::from("profile".as_bytes().to_owned()),
    )?;

    Ok(())
}

pub fn update_profile(update_profile: UpdateProfileInput) -> ExternResult<()> {
    //Validate did
    let (_did, did_hash) = did_validate_and_check_integrity(&update_profile.did, true)?;

    let profile_links = get_latest_link(
        did_hash.clone(),
        Some(LinkTag::from("profile".as_bytes().to_owned())),
    )
    .map_err(|error| err(format!("{}", error).as_ref()))?;

    match profile_links {
        Some(links) => {
            update_entry(
                get_header(links.target).map_err(|error| err(format!("{}", error).as_ref()))?,
                &Profile(update_profile.context, update_profile.profile),
            )?;
        }
        //No profile exists so we will just create one here
        None => {
            //Add profile entry
            let did_profile = Profile(update_profile.context, update_profile.profile);
            let did_profile_hash = hash_entry(&did_profile)?;
            create_entry(&did_profile)?;

            //Link profile entry to did
            create_link(
                did_hash,
                did_profile_hash,
                LinkTag::from("profile".as_bytes().to_owned()),
            )?;
        }
    };
    Ok(())
}

pub fn get_profile(did: DidInput) -> ExternResult<Option<Profile>> {
    //Validate did
    Uri::from_str(&did.0).map_err(|did_err| err(format!("{}", did_err.kind()).as_ref()))?;

    let profile_links = get_latest_link(
        hash_entry(Did(did.0))?,
        Some(LinkTag::from("profile".as_bytes().to_owned())),
    )
    .map_err(|error| err(format!("{}", error).as_ref()))?;

    match profile_links {
        Some(link) => {
            let entry = get_latest_entry(link.target, GetOptions::default())
                .map_err(|error| err(format!("{}", error).as_ref()))?;

            Ok(Some(try_from_entry::<Profile>(entry)?))
        }
        None => Ok(None),
    }
}

pub fn register_did(register_did: RegisterDidInput) -> ExternResult<()> {
    //Validate did
    let (did, did_hash) = did_validate_and_check_integrity(&register_did.did, false)?;

    //Validate incoming did document and its signed agent fields
    validate_did_doc(&register_did.did_document)?;

    //Add did entry
    create_entry(&did)?;

    //Add document entry
    let did_doc = DidDocument(register_did.did_document);
    let did_doc_hash = hash_entry(&did_doc)?;
    create_entry(&did_doc)?;

    //Link document entry to did
    create_link(
        did_hash.clone(),
        did_doc_hash,
        LinkTag::from("doc".as_bytes().to_owned()),
    )?;

    Ok(())
}

pub fn add_profile(add_profile: AddProfile) -> ExternResult<()> {
    //Validate did
    let (_did, did_hash) = did_validate_and_check_integrity(&add_profile.did, true)?;

    //Add profile entry
    let did_profile = Profile(add_profile.context, add_profile.profile);
    let did_profile_hash = hash_entry(&did_profile)?;
    create_entry(&did_profile)?;

    //Link profile entry to did
    create_link(
        did_hash,
        did_profile_hash,
        LinkTag::from("profile".as_bytes().to_owned()),
    )?;

    Ok(())
}
