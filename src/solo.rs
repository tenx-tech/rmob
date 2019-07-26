//! start sub-command

use crate::{BoxResult};
use crate::active_copirate::ActiveCoPirates;
use std::path::Path;

pub fn solo(repo_dir: &Path) -> BoxResult<()> {
    ActiveCoPirates::create_empty(repo_dir)?;

    println!("All th' gold shall be yers alone.");

    Ok(())
}


