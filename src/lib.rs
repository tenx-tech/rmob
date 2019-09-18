//! Command for including co-authors in Git commits when collaborating on code.

#![forbid(unsafe_code)]

use std::error::Error;
use std::path::PathBuf;

use git2::Repository;
use structopt::StructOpt;

mod active_copirate;
mod copirate;
mod embark;
mod prepare_commit_msg;
mod sail;
mod solo;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

pub const HOOK_PATH: &str = ".git/hooks/prepare-commit-msg";

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "Rmob", version = "0.1.0", author = "")]
struct Rmob {
    #[structopt(
        default_value = ".git-copirates",
        env = "GIT_COPIRATES_FILE",
        long = "git-copirates-file"
    )]
    git_copirates_file: String,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(StructOpt, Clone, Debug)]
enum CopirateSubcommand {
    /// Adds a co-pirate to the list
    #[structopt(name = "add")]
    Add {
        initials: String,
        name: String,
        email: String,
    },
    /// Removes co-pirate from the list
    #[structopt(name = "remove")]
    Remove {
        initials: String,
    },
}

#[derive(StructOpt, Clone, Debug)]
enum Command {
    /// Embark on rmob fer this git repo, call this once t' use rmob in yer git repo
    #[structopt(name = "embark")]
    Embark,
    /// Start pairin' or mobbin' by passin' a list of yer co-pirates te sail wit'
    // TODO: Accept only two-character input
    #[structopt(name = "sail")]
    Sail { copirates: Vec<String> },
    /// Sail solo (short fer `rmob sail solo`)
    #[structopt(name = "solo")]
    Solo,
    /// Edit copirates list
    #[structopt(name = "copirate")]
    Copirate {
        #[structopt(subcommand)]
        cmd: CopirateSubcommand
    },
    /// Called from the git hook only
    #[structopt(name = "prepare-commit-msg")]
    PrepareCommitMessage {
        #[structopt(parse(from_os_str))]
        commit_message_file: PathBuf,
    },
}

/// Executes the `rmob` application.
pub fn run() -> BoxResult<()> {
    let rmob = Rmob::from_args();

    let copirates_file = rmob.git_copirates_file;
    let repo = Repository::discover(".")?;
    let repo_dir = repo.workdir().ok_or("You're ON LAND, stupid.")?;

    match rmob.cmd {
        Command::Embark => embark::embark(repo_dir)?,
        Command::Solo => solo::solo(repo_dir)?,
        Command::Sail { ref copirates } if copirates == &["solo"] => solo::solo(repo_dir)?,
        Command::Sail { ref copirates } => sail::sail(&copirates_file, copirates, repo_dir)?,
        Command::Copirate { ref cmd } => {
            match cmd {
                CopirateSubcommand::Add { ref initials, ref name, ref email } => copirate::add(&copirates_file, initials, name, email)?,
                CopirateSubcommand::Remove { ref initials } => copirate::remove(&copirates_file, initials)?,
            };
        }

        Command::PrepareCommitMessage {
            commit_message_file,
        } => {
            prepare_commit_msg::inject_into_commit_message_file(&commit_message_file, repo_dir)?;
        }
    }

    Ok(())
}
