use hc_utils::{get_header, get_latest_entry, get_latest_link};
use hdk3::prelude::*;
use did_doc::Uri;
use std::str::FromStr;

use crate::did_validation::validate_did_doc;
use crate::utils::{did_validate_and_check_integrity, err, try_from_entry};
use crate::{
    AddProfile, CreateProfileInput, DidDocument, DidInput, Profile, RegisterDidInput,
    UpdateProfileInput, Did
};

pub fn create_profile(create_data: CreateProfileInput) -> ExternResult<()> {
    //Validate did
    let (did, did_hash) = did_validate_and_check_integrity(&create_data.did, false)?;

    //Validate incoming did document and its signed agent fields
    validate_did_doc(&create_data.did_document)?;

    //Add did entry
    create_entry(&did)?;

    //Add document entry
    let did_doc = DidDocument(create_data.did_document);
    let did_doc_hash = hash_entry(&did_doc)?;
    create_entry(&did_doc)?;

    //Link document entry to did
    create_link(
        did_hash.clone(),
        did_doc_hash,
        LinkTag::from("doc".as_bytes().to_owned()),
    )?;

    //Add profile entry
    let did_profile = Profile(create_data.profile);
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
                &Profile(update_profile.profile),
            )?;
        }
        //No profile exists so we will just create one here
        None => {
            //Add profile entry
            let did_profile = Profile(update_profile.profile);
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

    //Check that did is not already in the DHT
    let did = Did(did.0);
    let did_hash = hash_entry(&did)?;
    if get(did_hash.clone(), GetOptions)?.is_none() {
        return Ok(None)
    };

    let profile_links = get_latest_link(
        did_hash,
        Some(LinkTag::from("profile".as_bytes().to_owned())),
    )
    .map_err(|error| err(format!("{}", error).as_ref()))?;

    match profile_links {
        Some(link) => {
            let entry = get_latest_entry(link.target, GetOptions)
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
    let did_profile = Profile(add_profile.profile);
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
