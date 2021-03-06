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

pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => error("Could not convert entry"),
        },
        _ => error("Could not convert entry"),
    }
}
