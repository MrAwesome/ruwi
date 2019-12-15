extern crate assert_cmd;
extern crate ruwi;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_cli() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}
