//! Integration tests for the `qublis-qnetx-node` run and status subcommands.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_subcommand_help_shows_usage() {
    let mut cmd = Command::cargo_bin("qublis-qnetx-node").unwrap();
    cmd.arg("run")
       .arg("--help")
       .assert()
       .success()
       .stdout(predicate::str::contains("Run the validator node"));
}

#[test]
fn status_subcommand_help_shows_usage() {
    let mut cmd = Command::cargo_bin("qublis-qnetx-node").unwrap();
    cmd.arg("status")
       .arg("--help")
       .assert()
       .success()
       .stdout(predicate::str::contains("Query the current status"));
}
