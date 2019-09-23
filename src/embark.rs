//! Subcommand for initializing `rmob` with the given Git repository.

#[cfg(not(windows))]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::{fs, io::Result as IoResult};

use crate::{BoxResult, HOOK_PATH};

/// Registers the `rmob` Git hook with the Git repository located at `repo_dir`.
pub fn embark(repo_dir: &Path) -> BoxResult<()> {
    let hook_path = repo_dir.join(HOOK_PATH);
    if hook_path.exists() {
        return Err(Box::from("You have an existing prepare-commit-msg hook, which we need to overwrite. Please back it up and remove it!"));
    } else {
        create_hook(&hook_path)?;
    }

    Ok(())
}

fn create_hook(hook_file: &Path) -> IoResult<()> {
    let shell_name = if cfg!(not(windows)) { "bash" } else { "sh" };
    let hook_code = format!(
        "#!/bin/{}

rmob prepare-commit-msg \"$1\"",
        shell_name
    );

    write_executable(hook_file, &hook_code)?;

    println!("Success!");
    Ok(())
}

fn write_executable(file: &Path, contents: &str) -> IoResult<()> {
    #[cfg(not(windows))]
    {
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o775)
            .open(file)?;
    }

    #[cfg(windows)]
    {
        fs::OpenOptions::new().create(true).write(true).open(file)?;
    }

    fs::write(&file, contents)
}
