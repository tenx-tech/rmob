extern crate clap;

use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::fs;
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::process::Command;

type BoxResult = Result<(), Box<dyn Error>>;

fn main() -> BoxResult {
    let matches = App::new("Rmob")
        .version("0.1.0")
        .author("TenX Team <team@tenx.tech>")
        .about("Arr! Git mobbing in Rust")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize rmob for this git repo, call this once to use rmob in your git repo")
                .version("1.0"),
        )
        .subcommand(
            SubCommand::with_name("prepare-commit-msg")
                .about("Called from the git hook only")
                .version("1.0")
                .arg(
                    Arg::with_name("COMMIT_MESSAGE_FILE")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("prepare-commit-msg") {
        let commit_message_file = matches.value_of("COMMIT_MESSAGE_FILE").unwrap();
        inject_into_commit_message_file(commit_message_file)?;
    } else if let Some(_) = matches.subcommand_matches("init") {
        // TODO: Find the path to the top-level git hooks dir from anywhere, use libgit2?
        let hook_file = ".git/hooks/prepare-commit-msg";

        if Path::new(hook_file).exists() {
            // TODO: Want to bail! here, do I need a custom error type for that?
            panic!("You have an existing prepare-commit-msg hook, which we need to overwrite. Please back it up and remove it!");
        } else {
            create_hook(hook_file)?;
        }
    }

    Ok(())
}

fn create_hook(hook_file: &str) -> io::Result<()> {
    let hook_code = "#!/bin/bash

rmob prepare-commit-msg \"$1\"";

    write_executable(hook_file, hook_code)?;

    println!("Success!");
    Ok(())
}

// TODO: Make OS-agnostic
fn write_executable(file: &str, contents: &str) -> io::Result<()> {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o775)
        .open(file)
        .unwrap();
    fs::write(file, contents)
}

fn inject_into_commit_message_file(commit_message_file: &str) -> BoxResult {
    let commit_message = fs::read_to_string(commit_message_file)?;
    let mob_cmd = Command::new("git")
        .arg("mob-print")
        .output()?
        .clone()
        .stdout;
    let mob = String::from_utf8_lossy(&mob_cmd);
    let comment_pos = commit_message.find("# ").unwrap();
    let (git_message, git_comments) = commit_message.split_at(comment_pos);
    let updated_message = format!("{}{}{}", git_message, mob, git_comments);
    fs::write(commit_message_file, updated_message)?;

    Ok(())
}
