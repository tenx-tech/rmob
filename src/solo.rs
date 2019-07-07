//! start sub-command

extern crate dirs;
extern crate serde;
extern crate serde_json;

use crate::{BoxResult, ACTIVE_COPIRATES_FILE};

use serde::Deserialize;
use std::fs;
use std::collections::{HashMap};

#[derive(Deserialize, Debug)]
struct CoPirate {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct CoPirates {
    copirates: HashMap<String, CoPirate>,
}

pub fn solo() -> BoxResult {
    empty_copirates_file()?;

    println!("All th' gold shall be yers alone.");

    Ok(())
}

fn empty_copirates_file() -> BoxResult {
    fs::write(ACTIVE_COPIRATES_FILE, "")?;

    Ok(())
}

