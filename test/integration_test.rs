use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_default() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("Binary exists")
        .assert()
        .success()
        .sdtout(predicate::str::contains("Meow!"));
    Ok(())
}

#[test]
fn fail_on_nonexistent_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("Binary exists")
        .args(&["-f","no/such/file.text"])
        .assert()
        .failure();
    Ok(())
}