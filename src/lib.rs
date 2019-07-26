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
enum Rmob {
    /// Embark on rmob fer this git repo, call this once t' use rmob in yer git repo
    #[structopt(name = "embark")]
    Embark {},
    /// Start pairin' or mobbin' by passin' a list of yer co-pirates te sail wit'
    // TODO: Accept only two-character input
    #[structopt(name = "sail")]
    Sail { copirates: Vec<String> },
    /// Sail solo (short fer `rmob sail solo`)
    #[structopt(name = "solo")]
    Solo {},
    /// Called from the git hook only
    #[structopt(name = "prepare-commit-msg")]
    PrepareCommitMessage {
        #[structopt(parse(from_os_str))]
        commit_message_file: PathBuf,
    },
}

pub fn run() -> BoxResult<()> {
    let rmob = Rmob::from_args();

    let repo = Repository::discover(".")?;
    let repo_dir = repo.workdir().ok_or("You're ON LAND, stupid.")?;

    match rmob {
        Rmob::Embark {} => embark::embark(repo_dir)?,
        Rmob::Sail { copirates } => {
            if copirates == ["solo"] {
                solo::solo(repo_dir)?
            } else {
                sail::sail(&copirates, repo_dir)?
            }
        }
        Rmob::Solo {} => solo::solo(repo_dir)?,
        Rmob::PrepareCommitMessage {
            commit_message_file,
        } => {
            prepare_commit_msg::inject_into_commit_message_file(&commit_message_file, repo_dir)?;
        }
    }

    Ok(())
}
