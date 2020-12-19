use did_doc::Uri;
use hdk3::prelude::*;
use std::str::FromStr;

use crate::Did;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(err(reason))
}

pub fn err(reason: &str) -> HdkError {
    HdkError::Wasm(WasmError::Zome(String::from(reason)))
}

pub fn did_validate_and_check_integrity(did: &String) -> ExternResult<(Did, EntryHash)> {
    //Check that did is of valid syntax
    Uri::from_str(did).map_err(|did_err| err(format!("{}", did_err.kind()).as_ref()))?;

    //Check that did is not already in the DHT
    let did = Did(did.clone());
    let did_hash = hash_entry(&did)?;
    let did_check = get(did_hash.clone(), GetOptions)?;
    if did_check.is_none() {
        Err(err(
            "Did already exists please add profile using the add_profile function",
        ))
    } else {
        Ok((did, did_hash))
    }
}

pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => error("Could not convert entry"),
        },
        _ => error("Could not convert entry"),
    }
}
