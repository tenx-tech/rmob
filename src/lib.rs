use std::error::Error;
use std::path::{PathBuf};

use structopt::StructOpt;

mod copirate;
mod embark;
mod prepare_commit_msg;
mod sail;
mod solo;

pub const HOOK_NAME: &str = "prepare-commit-msg";
pub const COPIRATES_FILE: &str = ".git-copirates";
pub const ACTIVE_COPIRATES_FILE: &str = ".git/.git-rmob-template";

pub type BoxResult = Result<(), Box<dyn Error>>;

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

pub fn run() -> BoxResult {
    let rmob = Rmob::from_args();

    match rmob {
        Rmob::Embark {} => embark::embark()?,
        Rmob::Sail { copirates } => {
            if copirates == ["solo"] {
                solo::solo()?
            } else {
                sail::sail(&copirates)?
            }
        },
        Rmob::Solo {} => solo::solo()?,
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
