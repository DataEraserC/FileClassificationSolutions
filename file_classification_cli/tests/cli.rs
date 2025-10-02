use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_file_commands() {
    let mut cmd = Command::cargo_bin("file_classification_cli").unwrap();
    cmd.arg("file")
        .arg("create")
        .arg("--path")
        .arg("/tmp/test_file.txt")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("file_classification_cli").unwrap();
    let assert = cmd.arg("file").arg("list").assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(output.contains("/tmp/test_file.txt"));

    let mut cmd = Command::cargo_bin("file_classification_cli").unwrap();
    cmd.arg("file")
        .arg("delete")
        .arg("--path")
        .arg("/tmp/test_file.txt")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("file_classification_cli").unwrap();
    let assert = cmd.arg("file").arg("list").assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(!output.contains("/tmp/test_file.txt"));
}