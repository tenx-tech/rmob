use std::error::Error;
use std::path::PathBuf;

use structopt::StructOpt;

mod init;
mod prepare_commit_msg;

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
            prepare_commit_msg::inject_into_commit_message_file(
                commit_message_file
                    .to_str()
                    .ok_or("Ayyyr, what's on that hook laddy?")?,
            )?;
        }
    }

    Ok(())
}

