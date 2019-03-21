use assert_cmd::prelude::*;
use git2::Repository;
use rmob::*;
use std::process::Command;
use tempfile::{tempdir, TempDir};

#[test]
fn it_works() -> BoxResult {
    let dir = tempdir()?;
    let repo = Repository::init(dir.path())?;

    init(dir)?;

    // TODO: verify hook insertion works on both hashed and unhashed EDIT_COMMIT_MSG

    Ok(())
}

fn init(dir: TempDir) -> BoxResult {
    let mut rmob = Command::cargo_bin("rmob")?;
    rmob.current_dir(dir.path()).arg("init").assert().success();
    let hook = dir.path().join(".git/hooks/").join(HOOK_NAME);
    assert!(hook.exists());

    Ok(())
}
