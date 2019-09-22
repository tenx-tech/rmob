use std::{io, str::FromStr};

use crate::BoxResult;
use structopt::clap::{App, Shell};

/// Generates and outputs shell completion script for `rmob` for specific shell: bash, zsh, etc.
pub(crate) fn generate(app: &mut App, shell: &str) -> BoxResult<()> {
    let shell = Shell::from_str(shell)?;
    app.gen_completions_to("rmob", shell, &mut io::stdout());
    Ok(())
}
