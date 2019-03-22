use std::error::Error;
use std::fs;
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use structopt::StructOpt;

pub const HOOK_NAME: &str = "prepare-commit-msg";

pub type BoxResult = Result<(), Box<dyn Error>>;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "Rmob", version = "0.1.0", author = "")]
enum Rmob {
    /// Initialize rmob for this git repo, call this once to use rmob in your git repo
    #[structopt(name = "init")]
    Init {},
    /// Called from the git hook only
    #[structopt(name = "prepare-commit-msg")]
    PrepareCommitMessage {
        #[structopt(parse(from_os_str))]
        commit_message_file: PathBuf,
    },
}

pub fn run() -> BoxResult {
    let rmob = Rmob::from_args();

    match rmob {
        Rmob::Init {} => init()?,
        Rmob::PrepareCommitMessage {
            commit_message_file,
        } => {
            inject_into_commit_message_file(
                commit_message_file
                    .to_str()
                    .ok_or("Ayyyr, what's on that hook laddy?")?,
            )?;
        }
    }

    Ok(())
}

fn init() -> BoxResult {
    // TODO: Find the path to the top-level git hooks dir from anywhere, use libgit2?
        let hook_file = format!(".git/hooks/{}", HOOK_NAME);

    if Path::new(hook_file).exists() {
        // TODO: Want to bail! here, do I need a custom error type for that?
        panic!("You have an existing prepare-commit-msg hook, which we need to overwrite. Please back it up and remove it!");
    } else {
        create_hook(&hook_file)?;
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

pub fn inject_into_commit_message_file(commit_message_file: &str) -> BoxResult {
    let commit_message = fs::read_to_string(commit_message_file)?;
    let mob_cmd_output = Command::new("git").arg("mob-print").output()?;

    if !mob_cmd_output.status.success() {
        return Err(Box::from(
            String::from_utf8_lossy(&mob_cmd_output.stderr).into_owned(),
        ));
    }

    let mob = String::from_utf8_lossy(&mob_cmd_output.stdout);
    let comment_pos = commit_message
        .find("# ")
        .ok_or("No comments found in yer commit, landlover.")?;
    let (git_message, git_comments) = commit_message.split_at(comment_pos);
    let updated_message = format!("{}{}{}", git_message, mob, git_comments);
    fs::write(commit_message_file, updated_message)?;

    Ok(())
}
