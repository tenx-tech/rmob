//! init sub-command

use crate::{BoxResult, ACTIVE_COPIRATES_FILE};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::{fs, io};

pub fn init() -> BoxResult {
    // TODO: Find the path to the top-level git hooks dir from anywhere, use libgit2?
    let hook_file = ".git/hooks/prepare-commit-msg";
    let template_file = ACTIVE_COPIRATES_FILE;

    if Path::new(hook_file).exists() {
        // TODO: Want to bail! here, do I need a custom error type for that?
        panic!("You have an existing prepare-commit-msg hook, which we need to overwrite. Please back it up and remove it!");
    } else {
        create_hook(hook_file)?;
    }

    if !Path::new(template_file).exists() {
        fs::write(template_file, "")?;
    }

    Ok(())
}

pub fn create_hook(hook_file: &str) -> io::Result<()> {
    let hook_code = "#!/bin/bash

rmob prepare-commit-msg \"$1\"";

    write_executable(hook_file, hook_code)?;

    println!("Success!");
    Ok(())
}

// TODO: Make OS-agnostic
pub fn write_executable(file: &str, contents: &str) -> io::Result<()> {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o775)
        .open(file)?;
    fs::write(file, contents)
}
