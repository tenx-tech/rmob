//! start sub-command

use std::path::Path;

use crate::active_copirate::ActiveCoPirates;
use crate::BoxResult;

pub fn solo(repo_dir: &Path) -> BoxResult<()> {
    ActiveCoPirates::create_empty(repo_dir)?;

    println!("All th' gold shall be yers alone.");

    Ok(())
}
