use std::fs;
use std::io::prelude::*;
use crate::BoxResult;
use std::path::Path;
use crate::copirate::CoPirate;
use std::fs::OpenOptions;

const ACTIVE_COPIRATES_PATH: &str = ".git/.git-rmob-template";

#[derive(Debug)]
pub struct ActiveCoPirates {
    file: fs::File,
}

impl ActiveCoPirates {
    pub fn create_empty(repo_dir: &Path) -> BoxResult<ActiveCoPirates> {
        let active_copirates_path = repo_dir.join(ACTIVE_COPIRATES_PATH);
        fs::write(&active_copirates_path, "")?;

        let file = OpenOptions::new()
            .append(true)
            .open(active_copirates_path)?;

        Ok(ActiveCoPirates { file })
    }

    pub fn save(mut self, copirates: &[&CoPirate]) -> BoxResult<()> {
        for pirate in copirates {
            writeln!(self.file, "Co-authored-by: {}", pirate)?;
        }

        Ok(())
    }

    pub fn get(repo_dir: &Path) -> BoxResult<String> {
        let active_copirates = fs::read_to_string(repo_dir.join(ACTIVE_COPIRATES_PATH))?;

        Ok(active_copirates)
    }
}
