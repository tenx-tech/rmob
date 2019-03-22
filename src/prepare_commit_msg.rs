//! prepare-commit-msg subcommand

use crate::BoxResult;
use std::fs;
use std::process::Command;

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
