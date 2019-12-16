use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_iw_first_network_from_file() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.args(&[
        "-F",
        "src/samples/iw_two_different_networks.txt",
        "-c",
        "print",
        "-s",
        "iw",
        "-A",
        "first",
    ]);
    cmd.assert()
        .success()
        .stdout("Valparaiso_Guest_House 2\n")
        .stderr("[NOTE]: Selected network: \"Valparaiso_Guest_House 2\"\n");
}
