// Factor out active copirates as a session type with two states: empty and nonempty
use serde::Deserialize;
use std::fs;
use std::collections::HashMap;
use crate::BoxResult;
use std::path::Path;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

#[derive(Deserialize, Debug)]
pub struct CoPirate {
    pub name: String,
    pub email: String,
}

impl Display for CoPirate {
   fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
       write!(fmt, "{} <{}>", self.name, self.email)
   }
}

#[derive(Deserialize, Debug)]
pub struct CoPirates {
    pub copirates: HashMap<String, CoPirate>,
}

impl CoPirates {
    pub fn open(copirates_path: &Path) -> BoxResult<CoPirates> {
        let raw_copirates = fs::read_to_string(copirates_path)?;
        let existing_copirates = serde_json::from_str(&raw_copirates[..])?;

        Ok(existing_copirates)
    }

    pub fn get(&self, copirate: &String) -> BoxResult<&CoPirate> {
        let copirate = self.copirates.get(copirate).ok_or("Shiver me timbers! This be pirate be a stranger around these ports. Hint: Add it to ~/.git-copirates!")?;

        Ok(copirate)
    }

}

