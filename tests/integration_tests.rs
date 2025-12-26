use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() -> std::io::Result<()> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "cat.txt"])
        .assert()
        .failure();
    // .stdout(predicates::str::contains("Hello"));
    Ok(())
}
