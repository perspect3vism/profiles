use hc_utils::{get_header, get_latest_entry, get_latest_link};
use hdk3::prelude::*;

use crate::did_validation::validate_did_doc;
use crate::utils::{did_validate_and_check_integrity, err, try_from_entry};
use crate::{
    CreateProfileInput, DidDocument, DidInput, Profile, RegisterDidInput, UpdateProfileInput,
};

pub fn create_profile(create_data: CreateProfileInput) -> ExternResult<()> {
    //Validate did
    let (did, did_hash) = did_validate_and_check_integrity(&create_data.did)?;

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
    let (_did, did_hash) = did_validate_and_check_integrity(&update_profile.did)?;

    let profile_links = get_latest_link(
        did_hash,
        Some(LinkTag::from("profile".as_bytes().to_owned())),
    )
    .map_err(|error| err(format!("{}", error).as_ref()))?;

    match profile_links {
        Some(links) => {
            update_entry(
                get_header(links.target).map_err(|error| err(format!("{}", error).as_ref()))?,
                &Profile(update_profile.profile),
            )?;
            Ok(())
        }
        None => Err(err("You have no profile to update")),
    }
}

pub fn get_profile(did: DidInput) -> ExternResult<Option<Profile>> {
    //Validate did
    let (_did, did_hash) = did_validate_and_check_integrity(&did.0)?;

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
    let (did, did_hash) = did_validate_and_check_integrity(&register_did.did)?;

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

#[cfg(test)]
mod test_sigs {
    #[test]
    fn ed25519_pub_key_convertion() {
        //b58 decode to bytes
        let pub_key_decoded =
            bs58::decode("2SCkHJrXx1bfrABgf8phThpM5PFdq9Mf9PRrByaY2mtf").into_vec();
        assert_eq!(pub_key_decoded.is_ok(), true);
        //Try and create Ed25519 from this
        let pub_key = ed25519_dalek::PublicKey::from_bytes(&pub_key_decoded.unwrap());
        println!("{:?}", pub_key);
        assert_eq!(pub_key.is_ok(), true);
    }

    #[test]
    fn ecdsasecp256k1_pub_key_convertion() {
        //hex to bytes
        let pub_key_decoded = hex::decode(String::from(
            "033a6892d7dea6ce4ddc59d3d07f094e52b7c56763a0c07b74a0fff3a5c0c136ad",
        ));
        assert_eq!(pub_key_decoded.is_ok(), true);
        //Try and create secp256 key from this
        let pub_key = secp256k1::PublicKey::from_slice(&pub_key_decoded.unwrap());
        assert_eq!(pub_key.is_ok(), true);
    }
}
