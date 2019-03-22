//! start sub-command

extern crate dirs;
extern crate serde;
extern crate serde_json;

use crate::{BoxResult, COPIRATES_FILE};

use serde::Deserialize;
use std::fs;
use std::collections::HashMap;

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
    println!("{:?}", existing_copirates);

    // TODO Return Err if one of the copirates does not exist

    // TODO Add existing copirates to .git/.git-rmob-template

    Ok(())
}
