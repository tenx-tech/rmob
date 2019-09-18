use std::process::Command;

use assert_cmd::prelude::*;
use git2::{Commit, ObjectType, Repository};
use rmob::*;
use tempfile::{tempdir, TempDir};

#[test]
fn it_works() -> BoxResult<()> {
    let dir = tempdir()?;
    let repo = Repository::init(dir.path())?;

    embark(&dir)?;

    add_new_copirate(&dir)?;

    sail(&dir)?;

    init_git_repo(&dir)?;

    assert_oneliner_commit_message_contains_coauthors(&dir, &repo)?;

    assert_multiline_commit_message_contains_coauthors(&dir, &repo)?;

    Ok(())
}

fn embark(dir: &TempDir) -> BoxResult<()> {
    let mut rmob = Command::cargo_bin("rmob")?;
    rmob.current_dir(dir.path())
        .arg("embark")
        .assert()
        .success();
    let hook = dir.path().join(HOOK_PATH);
    assert!(hook.exists());

    Ok(())
}

fn add_new_copirate(dir: &TempDir) -> BoxResult<()> {
    let mut rmob = Command::cargo_bin("rmob")?;
    rmob.current_dir(dir.path())
        .arg("--git-copirates-file")
        .arg(dir.path().join(".git-copirates"))
        .arg("copirate")
        .arg("add")
        .arg("cb")
        .arg("'Charlotte de Berry'")
        .arg("'berrydeath@england.pir'")
        .assert()
        .success();

    Ok(())
}

fn sail(dir: &TempDir) -> BoxResult<()> {
    let mut rmob = Command::cargo_bin("rmob")?;
    rmob.current_dir(dir.path())
        .arg("--git-copirates-file")
        .arg(dir.path().join(".git-copirates"))
        .arg("sail")
        .arg("cb")
        .assert()
        .success();

    Ok(())
}

fn init_git_repo(dir: &TempDir) -> BoxResult<()> {
    Command::new("git")
        .current_dir(dir.path())
        .arg("init")
        .assert()
        .success();

    Ok(())
}

fn assert_oneliner_commit_message_contains_coauthors(
    dir: &TempDir,
    repo: &Repository,
) -> BoxResult<()> {
    Command::new("git")
        .current_dir(dir.path())
        .arg("commit")
        .arg("-m")
        .arg("Arrrrrr!")
        .arg("--allow-empty")
        .assert()
        .success();

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

fn assert_multiline_commit_message_contains_coauthors(
    dir: &TempDir,
    repo: &Repository,
) -> BoxResult<()> {
    const MULTILINE_MESSAGE: &str = r#"
# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch integration-test
# Changes not staged for commit:
#	modified:   tests/integration_tests.rs
#
    "#;
    Command::new("git")
        .current_dir(dir.path())
        .arg("commit")
        .arg("-m")
        .arg(MULTILINE_MESSAGE)
        .arg("--allow-empty")
        .assert()
        .success();

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

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}
