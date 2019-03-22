use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use structopt::StructOpt;

mod init;

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
        Rmob::Init {} => init::init()?,
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
