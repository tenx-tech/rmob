//! prepare-commit-msg subcommand

use crate::{BoxResult, ACTIVE_COPIRATES_FILE};
use std::fs;

pub fn inject_into_commit_message_file(commit_message_file: &str) -> BoxResult {
    const PATTERN: &str = "\n# ";

    let commit_message = fs::read_to_string(commit_message_file)?;
    let mob = fs::read_to_string(ACTIVE_COPIRATES_FILE)?;
    // Can I transform that into .unwrap_or_else()
    let comment_pos = if let Some(message) = commit_message.find(PATTERN) {
        message
    } else {
        commit_message.len().saturating_sub(1)
    };
    let (git_message, git_comments) = commit_message.split_at(comment_pos);
    let updated_message = format!("{}\n\n{}{}", git_message, mob, git_comments);

    fs::write(commit_message_file, updated_message)?;

    Ok(())
}
