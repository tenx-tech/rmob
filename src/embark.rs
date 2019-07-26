//! embark sub-command

use crate::{BoxResult, HOOK_PATH};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::{fs, io};

pub fn embark(repo_dir: &Path) -> BoxResult<()> {
    let hook_path = repo_dir.join(HOOK_PATH);
    if hook_path.exists() {
        return Err(Box::from("You have an existing prepare-commit-msg hook, which we need to overwrite. Please back it up and remove it!"));
    } else {
        create_hook(&hook_path)?;
    }

    Ok(())
}

pub fn create_hook(hook_file: &Path) -> io::Result<()> {
    let hook_code = "#!/bin/bash

rmob prepare-commit-msg \"$1\"";

    write_executable(hook_file, hook_code)?;

    println!("Success!");
    Ok(())
}

// TODO: Make OS-agnostic
pub fn write_executable(file: &Path, contents: &str) -> io::Result<()> {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o775)
        .open(file)?;
    fs::write(file, contents)
}
