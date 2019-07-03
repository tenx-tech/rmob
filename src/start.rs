//! start sub-command

extern crate dirs;
extern crate serde;
extern crate serde_json;

use crate::{BoxResult, COPIRATES_FILE};

use serde::Deserialize;
use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Deserialize, Debug)]
struct CoPirate {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct CoPirates {
    copirates: HashMap<String, CoPirate>,
}

pub fn start(copirates: &[String]) -> BoxResult {
    let ship = dirs::home_dir().ok_or("Could not find yer ship oy!")?;
    let raw_copirates = fs::read_to_string(ship.join(COPIRATES_FILE))?;
    let existing_copirates: CoPirates = serde_json::from_str(&raw_copirates[..])?;

    fail_if_pirate_not_present(copirates, existing_copirates)?;

    // TODO Add existing copirates to .git/.git-rmob-template

    Ok(())
}

fn fail_if_pirate_not_present(copirates: &[String], existing_copirates: CoPirates) -> BoxResult {
    let existing_copirates_hashset: HashSet<&String> = existing_copirates.copirates.keys().collect();
    let copirates_hashset: HashSet<&String> = copirates.into_iter().collect();
    if existing_copirates_hashset.is_disjoint(&copirates_hashset) {
        return Err(Box::from("We didn't recognize this pirate's initials. Please add to your ~/.git-copirates file!"));
    }

    Ok(())
}
