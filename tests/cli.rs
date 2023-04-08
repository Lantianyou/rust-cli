use std::process::Command;

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

#[test]
fn works() {
    assert!(true)
}

#[test]
fn runs() {
    // let mut cmd = Command::new("ls");
    // let res = cmd.output();
    // assert!(res.is_ok());

    // let mut cmd = Command::cargo_bin("cli").unwrap();
    // cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
