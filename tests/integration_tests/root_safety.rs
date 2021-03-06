use rexpect::errors::Result;
use rexpect::spawn_bash;
use tempfile;

use super::utils::{get_unguarded_cmd_with_args, UNGUARDED_TIMEOUT_MS};

const BINARY_CREATED_TOKEN: &str = "CREATED_YOUR_BINARY_MLORD";

#[test]
fn test_malicious_interface_determinator_binary() -> Result<()> {
    impl_test_malicious_binary("ip", "wired connect")
}

#[test]
fn test_malicious_selection_binary() -> Result<()> {
    impl_test_malicious_binary("fzf", "-m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print")
}

fn get_full_malicious_binary_name(malicious_dir: &str, malicious_binary_name: &str) -> String {
    format!("{}/{}", malicious_dir, malicious_binary_name)
}

fn get_shell_cmd_for_creating_malicious_binary_named(full_filename: &str) -> String {
    format!(
        "echo -e '#!/bin/bash\necho I AM A MALICIOUS BINARY' > {} && chmod +x {} && echo {}",
        full_filename, full_filename, BINARY_CREATED_TOKEN
    )
}

fn impl_test_malicious_binary(malicious_binary_name: &str, ruwi_args: &str) -> Result<()> {
    let mut p = spawn_bash(UNGUARDED_TIMEOUT_MS)?;

    let tmpdir = tempfile::tempdir().unwrap();

    let malicious_dir = tmpdir.path().as_os_str().to_str().unwrap();
    dbg!(&malicious_dir);

    p.execute(
        &format!("export PATH=\"{}:$PATH\" && echo PATH_SET_UP", malicious_dir),
        "PATH_SET_UP",
    )?;

    p.send_line("echo $PATH")?;
    p.exp_string(malicious_dir)?;
    p.wait_for_prompt()?;


    let full_malicious_binary_name = get_full_malicious_binary_name(malicious_dir, malicious_binary_name);
    let create_bin_cmd = get_shell_cmd_for_creating_malicious_binary_named(&full_malicious_binary_name);
    p.execute(&create_bin_cmd, BINARY_CREATED_TOKEN)?;
    p.wait_for_prompt()?;

    p.send_line("export PRETEND_TO_BE_ROOT=1")?;
    p.wait_for_prompt()?;

    let compromised_cmd = format!(
        "{} || echo COMMAND_FAILED_YESSS",
        get_unguarded_cmd_with_args(ruwi_args),
    );

    p.execute(&compromised_cmd, &format!("{:?}", ruwi::errors::RuwiErrorKind::BinaryWritableByNonRootWhenRunningAsRoot))?;
    p.exp_regex(&format!("external binary .{}.", &full_malicious_binary_name))?;
    p.exp_string("COMMAND_FAILED_YESSS")?;
    p.wait_for_prompt()?;

    p.send_line(&format!("rm -r {} && echo \"DELETED_DIR\"", &malicious_dir))?;
    p.exp_string("DELETED_DIR")?;
    p.wait_for_prompt()?;
    Ok(())
}
