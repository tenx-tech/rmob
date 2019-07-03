//! start sub-command

extern crate dirs;
extern crate serde;
extern crate serde_json;

use crate::{BoxResult, COPIRATES_FILE, ACTIVE_COPIRATES_FILE};

use serde::Deserialize;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct CoPirate {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct CoPirates {
    copirates: HashMap<String, CoPirate>,
}

pub fn sail(copirates: &[String]) -> BoxResult {
    let ship = dirs::home_dir().ok_or("Could not find yer ship oy!")?;
    let raw_copirates = fs::read_to_string(ship.join(COPIRATES_FILE))?;
    let existing_copirates: CoPirates = serde_json::from_str(&raw_copirates[..])?;

    fail_if_pirate_not_present(copirates, &existing_copirates)?;

    empty_copirates_file()?;

    save_copirates(copirates, existing_copirates)?;

    Ok(())
}

fn save_copirates(copirates: &[String], existing_copirates: CoPirates) -> BoxResult {
    let mut file = OpenOptions::new()
        .append(true)
        .open(ACTIVE_COPIRATES_FILE)
        .unwrap();
    for pirate in copirates {
        let existing_pirate = existing_copirates.copirates.get(pirate).ok_or("Wait what Sally it was right there?")?;
        writeln!(file, "Co-authored-by: {} <{}>", existing_pirate.name, existing_pirate.email)?;
    }

    Ok(())
}

fn empty_copirates_file() -> BoxResult {
    fs::write(ACTIVE_COPIRATES_FILE, "")?;

    Ok(())
}

fn fail_if_pirate_not_present(copirates: &[String], existing_copirates: &CoPirates) -> BoxResult {
    let existing_copirates: HashSet<&String> = existing_copirates.copirates.keys().collect();
    let copirates: HashSet<&String> = copirates.into_iter().collect();
    if !copirates.is_subset(&existing_copirates) {
        return Err(Box::from("We didn't recognize this pirate's initials. Please add to your ~/.git-copirates file!"));
    }

    Ok(())
}
