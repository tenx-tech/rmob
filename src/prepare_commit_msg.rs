//! prepare-commit-msg subcommand

use crate::{BoxResult, ACTIVE_COPIRATES_FILE};
use std::fs;

pub fn inject_into_commit_message_file(commit_message_file: &str) -> BoxResult {
    let commit_message = fs::read_to_string(commit_message_file)?;
    let mob = fs::read_to_string(ACTIVE_COPIRATES_FILE)?;
    let comment_pos = commit_message
        .find("# ")
        .ok_or("No comments found in yer commit, landlover.")?;
    let (git_message, git_comments) = commit_message.split_at(comment_pos);
    let updated_message = format!("{}{}{}", git_message, mob, git_comments);

    fs::write(commit_message_file, updated_message)?;

    Ok(())
}
