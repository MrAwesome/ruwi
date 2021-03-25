use rexpect::errors::Result;

use rexpect::spawn_bash;
use super::utils::*;

// NOTE: spawn_bash is being used because fzf's visible output is often not seen by rexpect's spawn
//       (it is still not seen by spawn_bash, but some pieces of output are)

#[test]
fn test_iw_connect_with_fzf_search_string() -> Result<()> {
    let cmd = get_dryrun_cmd_with_args_with_env(
        "-m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print",
        &[("RUWI_FZF_SEARCH_STRING", "alltheint")]
    );
    let mut p = spawn_bash(DRYRUN_TIMEOUT_MS)?;
    p.execute(&cmd, "Selected network")?;
    p.exp_string("alltheinternets")?;
    p.exp_string("alltheinternets")?;
    Ok(())
}

#[test]
fn test_iw_fourth_of_many_networks_with_fzf_select() -> Result<()> {
    let cmd = get_dryrun_cmd_with_args("-m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw select");
    let mut p = spawn_bash(DRYRUN_TIMEOUT_MS)?;
    p.execute(&cmd, "Running in dryrun")?;
    p.send_control('n')?;
    p.send_control('n')?;
    p.send_control('n')?;
    p.send_control('m')?;
    p.exp_string("Selected network: \"alltheinternets\"")?;
    p.exp_regex("\r\nalltheinternets\r\n")?;
    p.wait_for_prompt()?;
    Ok(())
}


#[test]
fn test_fzf_exit_codes() -> Result<()> {
    let cmd = get_dryrun_cmd_with_args(
        "-m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print",
    );
    let mut p = spawn_bash(DRYRUN_TIMEOUT_MS)?;

    // Ctrl-c run:
    p.execute(&cmd, "Running in dryrun")?;
    p.send_control('c')?;

    // If this fails, Ctrl-c failed to kill fzf/ruwi
    p.wait_for_prompt()?;

    // If this fails, Ctrl-c during fzf did not cause ruwi to exit with the correct error code (1)
    p.execute("echo YEET${?}YEET", "YEET1YEET")?;

    // Normal run:
    p.execute(&cmd, "Running in dryrun")?;
    p.send_control('m')?;

    // If this fails, ruwi did not exit upon selection of a network in fzf
    p.wait_for_prompt()?;

    // If this fails, a normal selection in ruwi caused a non-zero exit code
    p.execute("echo YEET${?}YEET", "YEET0YEET")?;

    Ok(())
}

#[test]
fn test_fzf_respects_ctrl_r_refresh() -> Result<()> {
    let cmd = get_dryrun_cmd_with_args(
        "-m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print",
    );
    let mut p = spawn_bash(DRYRUN_TIMEOUT_MS)?;
    p.execute(&cmd, "Running in dryrun")?;
    p.send_control('r')?;
    p.exp_string("Refresh requested")?;
    p.send_control('m')?;
    p.exp_string("Patrician Pad")?;
    p.wait_for_prompt()?;
    Ok(())
}

#[test]
fn test_fzf_respects_refresh_string_refresh() -> Result<()> {
    let cmd = get_dryrun_cmd_with_args(
        "-m fzf wifi -F src/parse/samples/iw_many_networks.txt -s iw connect -c print",
    );
    let mut p = spawn_bash(DRYRUN_TIMEOUT_MS)?;
    p.execute(&cmd, "Running in dryrun")?;

    // We would rather send_line("refresh"), but fzf doesn't seem to see characters we send it
    // This locks "refresh" in as the bottom option. If others need to be below it, insert the
    // appropriate number of ctrl-p before the ctrl-m.
    {
        for _ in 0..50 {
            p.send_control('n')?;
        }
        p.send_control('m')?;
    }
    p.exp_string("Refresh requested")?;
    p.send_control('n')?;
    p.send_control('m')?;
    p.exp_string("casa")?;
    p.wait_for_prompt()?;
    Ok(())
}
