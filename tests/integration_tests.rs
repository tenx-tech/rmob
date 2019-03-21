use assert_cmd::prelude::*;
use git2::Repository;
use rmob::*;
use std::error::Error;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn it_works() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    let repo = Repository::init(dir.path())?;

    let mut rmob = Command::cargo_bin("rmob")?;
    rmob.current_dir(dir.path()).arg("init").assert().success();
    let hook = dir.path().join(".git/hooks/").join(HOOK_NAME);
    assert!(hook.exists());

    Ok(())
}
