extern crate clap;

use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::os::unix::fs::OpenOptionsExt;

fn main() {
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

    if let Some(matches) = matches.subcommand_matches("init") {
        // TODO: Find the path to the top-level git hooks dir from anywhere, use libgit2?
        let hook_file = ".git/hooks/prepare-commit-msg";

        if Path::new(hook_file).exists() {
            println!("You have an existing prepare-commit-msg hook, which we need to overwrite. Please back it up and remove it!");
        } else {
            let hook_code = "#!/bin/bash

rmob prepare-commit-msg \"$1\"";

            // TODO: Make OS-agnostic
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o775)
                .open(hook_file)
                .unwrap();

            fs::write(hook_file, hook_code)
                .expect("Failed to write git hook");


            println!("Success!");
        }
    }
}
