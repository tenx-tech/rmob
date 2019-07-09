use serde::Deserialize;
use std::fs;
use std::collections::{HashMap};
use crate::{BoxResult, ACTIVE_COPIRATES_FILE};

#[derive(Deserialize, Debug)]
pub struct CoPirate {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct CoPirates {
    pub copirates: HashMap<String, CoPirate>,
}

impl CoPirates {
    pub fn empty_copirates_file() -> BoxResult {
        fs::write(ACTIVE_COPIRATES_FILE, "")?;

        Ok(())
    }
}

