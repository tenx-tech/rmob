use std::fs;
use std::process::Command;
use std::os::unix::fs::OpenOptionsExt;

pub fn create_hook(hook_file: &str) {
    let hook_code = "#!/bin/bash

rmob prepare-commit-msg \"$1\"";

    write_executable(hook_file, hook_code);

    println!("Success!");
}

// TODO: Make OS-agnostic
pub fn write_executable(file: &str, contents: &str) {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o775)
        .open(file)
        .unwrap();
    fs::write(file, contents).expect("Failed to write git hook");
}

pub fn inject_into_commit_message_file(commit_message_file: &str) {
    let commit_message =
        fs::read_to_string(commit_message_file).expect("Couldn't read git's COMMIT_EDITMSG");
    let mob_cmd = Command::new("git")
        .arg("mob-print")
        .output()
        .expect("for now we still depend on git mob-print")
        .clone()
        .stdout;
    let mob = String::from_utf8_lossy(&mob_cmd);
    let comment_pos = commit_message.find("# ").unwrap();
    let (git_message, git_comments) = commit_message.split_at(comment_pos);
    let updated_message = format!("{}{}{}", git_message, mob, git_comments);
    fs::write(commit_message_file, updated_message).expect("Error writing commit message file");
}
