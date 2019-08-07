//! Subcommand to disable mobbing for the active Git repository.

use std::path::Path;

use crate::active_copirate::ActiveCoPirates;
use crate::BoxResult;

/// Disable mobbing on the Git repository located at `repo_dir`.
pub fn solo(repo_dir: &Path) -> BoxResult<()> {
    ActiveCoPirates::create_empty(repo_dir)?;

    println!("All th' gold shall be yers alone.");

    Ok(())
}
