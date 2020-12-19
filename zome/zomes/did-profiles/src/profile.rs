use did_doc::{
    fields::{PublicKeyEncoding, PublicKeyType},
    Uri,
};
use ed25519_dalek::Verifier;
use hc_utils::{get_header, get_latest_entry, get_latest_link};
use hdk3::prelude::*;
use secp256k1::Secp256k1;
use std::{collections::BTreeMap, str::FromStr};

use crate::utils::{did_validate_and_check_integrity, err, try_from_entry};
use crate::{CreateProfileInput, DidDocument, DidInput, Profile, UpdateProfileInput};

pub fn create_profile(create_data: CreateProfileInput) -> ExternResult<()> {
    //Validate did
    let (did, did_hash) = did_validate_and_check_integrity(&create_data.did)?;

    //Look for pub keys are of the type and encoding that we support (Ed25519VerificationKey2018 base58 & EcdsaSecp256k1VerificationKey2019 hex)
    let pub_key = create_data.did_document.public_key().iter().find(|key| {
        (key.encoding() == PublicKeyEncoding::Hex
            && key.kind() == PublicKeyType::EcdsaSecp256k1VerificationKey2019)
            || (key.encoding() == PublicKeyEncoding::Base58
                && key.kind() == PublicKeyType::Ed25519VerificationKey2018)
    }).ok_or(err("Public key not found with type Ed25519VerificationKey2018 or EcdsaSecp256k1VerificationKey2019"))?;

    //Verify that the signed agent is in the document
    if !create_data.did_document.extra.contains_key("signed_agent") {
        Err(err("No signed_agent field in the did document"))?;
    };
    //Get bytes of current agent
    let agent_pub = agent_info()?.agent_latest_pubkey.to_string();
    let agent_bytes = agent_pub.as_bytes();

    //Validate that the signed_agent signature is valid
    match pub_key.kind() {
        PublicKeyType::EcdsaSecp256k1VerificationKey2019 => {
            //Get the pub key ready
            let pub_key_decoded = hex::decode(pub_key.data());
            let pub_key = secp256k1::PublicKey::from_slice(&pub_key_decoded.unwrap())
                .map_err(|_| err("Could not create scep256 pub key from did document data"))?;

            //Verify the sig
            let secp = Secp256k1::verification_only();
            secp.verify(
                &secp256k1::Message::from_slice(&agent_bytes)
                    .map_err(|_| err("Error converting agent bytes to secp message"))?,
                &secp256k1::Signature::from_str(
                    &create_data
                        .did_document
                        .extra
                        .get("signed_agent")
                        .unwrap()
                        .to_string(),
                )
                .map_err(|_| err("Could not convert signed_agent data to scep signature"))?,
                &pub_key,
            )
            .map_err(|_| err("Signed agent is not valid"))?;
        }
        PublicKeyType::Ed25519VerificationKey2018 => {
            //Get the pub key ready
            let pub_key_decoded = bs58::decode(pub_key.data())
                .into_vec()
                .map_err(|_| err("Could not decode pub key from base58"))?;
            let public_key = ed25519_dalek::PublicKey::from_bytes(&pub_key_decoded)
                .map_err(|_| err("Public key not of ed25519 format"))?;

            //Verify the sig
            public_key
                .verify(
                    agent_bytes,
                    &ed25519_dalek::Signature::try_from(
                        create_data
                            .did_document
                            .extra
                            .get("signed_agent")
                            .unwrap()
                            .to_string()
                            .as_bytes(),
                    )
                    .map_err(|_| err("Signed agent not correct format"))?,
                )
                .map_err(|_| err("Signed agent is not valid"))?;
        }
        _ => unreachable!(),
    }

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
