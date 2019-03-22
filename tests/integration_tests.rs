use assert_cmd::prelude::*;
use git2::{Commit, ObjectType, Oid, Repository, Signature};
use rmob::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::{tempdir, TempDir};

#[test]
fn it_works() -> BoxResult {
    let dir = tempdir()?;
    let repo = Repository::init(dir.path())?;

    init(&dir)?;

    // TODO: Set up rmob and expect those co-pirates

    assert_plain_commit_message_contains_coauthors(dir, &repo)?;

    // TODO: Test commit message with hashes

    Ok(())
}

fn init(dir: &TempDir) -> BoxResult {
    let mut rmob = Command::cargo_bin("rmob")?;
    rmob.current_dir(dir.path()).arg("init").assert().success();
    let hook = dir.path().join(".git/hooks/").join(HOOK_NAME);
    assert!(hook.exists());

    Ok(())
}

fn assert_plain_commit_message_contains_coauthors(dir: TempDir, repo: &Repository) -> BoxResult {
    let arr_file = dir.path().join("arrfile");
    fs::write(&arr_file, "arr")?;
    add_and_commit(&repo, Path::new("arrfile"), "Arrrrrr!")?;
    let commit = find_last_commit(&repo)?;
    assert!(
        commit
            .message()
            .ok_or("no commit message")?
            .contains("Co-authored-by"),
        "Did not include the Co-Author for a commit message without comments (hashes)"
    );

    Ok(())
}

fn add_and_commit(repo: &Repository, path: &Path, message: &str) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    index.add_path(path)?;
    let oid = index.write_tree()?;
    let signature = Signature::now("Mister Sinister", "mister@sinister.net")?;
    let tree = repo.find_tree(oid)?;
    repo.commit(
        Some("HEAD"), //  point HEAD to our new commit
        &signature,   // author
        &signature,   // committer
        message,      // commit message
        &tree,        // tree
        &[],
    ) // parents
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}
