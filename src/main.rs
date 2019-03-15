extern crate clap;

use clap::{App, Arg, SubCommand};
use std::path::Path;

use rmob::*;

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
        inject_into_commit_message_file(commit_message_file);
    }

    if let Some(matches) = matches.subcommand_matches("init") {
        // TODO: Find the path to the top-level git hooks dir from anywhere, use libgit2?
        let hook_file = ".git/hooks/prepare-commit-msg";

        if Path::new(hook_file).exists() {
            println!("You have an existing prepare-commit-msg hook, which we need to overwrite. Please back it up and remove it!");
        } else {
            create_hook(hook_file);
        }
    }
}

