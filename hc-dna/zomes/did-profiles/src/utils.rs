use did_doc::Uri;
use hdk::prelude::*;
use std::str::FromStr;

use crate::Did;

pub fn err(reason: &str) -> WasmError {
    WasmError::Host(String::from(reason))
}

pub fn did_validate_and_check_integrity(
    did: &String,
    should_exist: bool,
) -> ExternResult<(Did, EntryHash)> {
    //Check that did is of valid syntax
    Uri::from_str(did).map_err(|did_err| err(format!("{}", did_err.kind()).as_ref()))?;

    //Check for did in DHT
    let did = Did(did.clone());
    let did_hash = hash_entry(&did)?;
    let did_check = get(did_hash.clone(), GetOptions::default())?;

    if should_exist {
        if did_check.is_some() {
            Ok((did, did_hash))
        } else {
            Err(err("Given DID does not exist"))
        }
    } else {
        if did_check.is_none() {
            Ok((did, did_hash))
        } else {
            Err(err(
                "Given did already exists in the DHT. Expected a unique DID.",
            ))
        }
    }
}
