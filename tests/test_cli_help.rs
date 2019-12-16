use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_cli_help() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}
