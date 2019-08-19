//! Subcommand to begin mobbing on the active Git repository.

use std::path::Path;

use crate::active_copirate::ActiveCoPirates;
use crate::copirate::CoPirates;
use crate::BoxResult;

/// Begin mobbing with the given co-authors on the Git repository located at `repo_dir`.
pub fn sail(copirates_file: &str, copirates: &[String], repo_dir: &Path) -> BoxResult<()> {
    let ship = dirs::home_dir().ok_or("Could not find yer ship oy!")?;
    let existing_copirates = CoPirates::open(&ship.join(copirates_file))?;

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
