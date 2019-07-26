//! start sub-command

extern crate dirs;
use crate::BoxResult;

use crate::active_copirate::ActiveCoPirates;
use crate::copirate::CoPirates;
use std::path::Path;

const COPIRATES_PATH: &str = ".git-copirates";

pub fn sail(copirates: &[String], repo_dir: &Path) -> BoxResult<()> {
    let ship = dirs::home_dir().ok_or("Could not find yer ship oy!")?;
    let existing_copirates = CoPirates::open(&ship.join(COPIRATES_PATH))?;

    save_copirates(copirates, existing_copirates, repo_dir)?;

    Ok(())
}

fn save_copirates(
    copirates: &[String],
    existing_copirates: CoPirates,
    repo_dir: &Path,
) -> BoxResult<()> {
    let copirates: Vec<_> = copirates
        .iter()
        .map(|initial| existing_copirates.get(initial))
        .collect::<Result<_, _>>()?;

    let active_copirates = ActiveCoPirates::create_empty(repo_dir)?;
    active_copirates.save(&copirates)?;

    println!("Sail away!");

    Ok(())
}
