use std::fs;
use std::process::Command;
use std::os::unix::fs::OpenOptionsExt;
use std::io;
use std::error::Error;

pub const HOOK_NAME: &str = "prepare-commit-msg";

pub type BoxResult = Result<(), Box<dyn Error>>;

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

pub fn inject_into_commit_message_file(commit_message_file: &str) -> BoxResult {
    let commit_message = fs::read_to_string(commit_message_file)?;
    let mob_cmd_output = Command::new("git")
        .arg("mob-print")
        .output()?;

    if !mob_cmd_output.status.success() {
        return Err(Box::from(String::from_utf8_lossy(&mob_cmd_output.stderr).into_owned()));
    }

    let mob = String::from_utf8_lossy(&mob_cmd_output.stdout);
    let comment_pos = commit_message.find("# ").ok_or("No comments found in yer commit, landlover.")?;
    let (git_message, git_comments) = commit_message.split_at(comment_pos);
    let updated_message = format!("{}{}{}", git_message, mob, git_comments);
    fs::write(commit_message_file, updated_message)?;

    Ok(())
}

