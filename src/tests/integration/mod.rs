use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_new() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nebulis-cli")?;

    cmd.arg("new").arg("test_project");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Creating Nebulis project"));

    Ok(())
}
