//! start sub-command

use crate::{BoxResult};
use crate::copirate::CoPirates;

pub fn solo() -> BoxResult {
    CoPirates::empty_copirates_file()?;

    println!("All th' gold shall be yers alone.");

    Ok(())
}


