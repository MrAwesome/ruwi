use rexpect::errors::*;
use rexpect::spawn_bash;

// An absolutely end-to-end test of ruwi:
// * Downloads and starts a Linux VM
// * Creates virtual wifi radios
// * Starts access points 
// * Connects to them using ruwi.
#[test]
#[ignore]
fn test_full_integration_with_archlinux_vm() -> Result<()> {
    let mut shell_for_copying_files_into_shared_dir = spawn_bash(Some(200))?;

    // TODO: Other targets as well
    eprintln!("[TEST]: Copying files into place...");
    shell_for_copying_files_into_shared_dir.execute(
        "mkdir -p /tmp/archlinux/shared && cp -r ./target/debug/ruwi ./ci/configs/*.conf /tmp/archlinux/shared && echo COPIEDDD",
        "COPIEDDD",
    )?;
    shell_for_copying_files_into_shared_dir.wait_for_prompt()?;
    eprintln!("[TEST]: Done copying files.");

    // TODO: make this work when offline and the iso is present
    // TODO: update to the newest iso always?
    eprintln!("[TEST]: Fetching/checking LiveCD...");
    let mut shell_for_fetching_iso = spawn_bash(Some(900_000))?;
    let command = concat!(
        "cd /tmp/archlinux/ && ",
        "curl http://mirror.rackspace.com/archlinux/iso/2019.12.01/md5sums.txt | grep archlinux-2019.12.01-x86_64.iso | md5sum -c || ",
        "curl -O http://mirror.rackspace.com/archlinux/iso/2019.12.01/archlinux-2019.12.01-x86_64.iso && ",
        "curl http://mirror.rackspace.com/archlinux/iso/2019.12.01/md5sums.txt | grep archlinux-2019.12.01-x86_64.iso | md5sum -c && ",
        "echo DOWNLOADEDBRAH || ",
        "exit 1"
    );
    shell_for_fetching_iso.execute(&command, "DOWNLOADEDBRAH")?;
    eprintln!("[TEST]: Successfully downloaded/checksummed LiveCD!");

    eprintln!("[TEST]: Starting VM.");
    let mut p = spawn_bash(Some(90000))?;
    p.execute(
        "qemu-system-x86_64 -cdrom /tmp/archlinux/archlinux-2019.12.01-x86_64.iso -m 1024 -enable-kvm -nographic -vga none -virtfs local,path=/tmp/archlinux/shared,mount_tag=host0,security_model=passthrough,id=host0", 
        "Arch Linux"
    )?;
    eprintln!("[TEST]: Reached kernel options...");
    p.send_control('i')?;
    p.exp_string("vmlinuz")?;
    p.send_line(" console=ttyS0")?;
    eprintln!("[TEST]: Started boot...");

    p.exp_string("archiso login:")?;
    eprintln!("[TEST]: Successfully reached prompt!");

    p.send_line("root")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Successfully logged in!");

    eprintln!("[TEST]: Installing packages...");
    p.send_line("pacman -Sy --noconfirm hostapd && echo INSTALLEDITYES")?;
    p.exp_string("INSTALLEDITYES")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Successfully installed packages!");
    p.send_line("modprobe mac80211_hwsim radios=3 && echo MODPROBED")?;
    p.exp_string("MODPROBED")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Successfully loaded virtual radios!");
    p.send_line("mkdir -p /tmp/host_shared && echo 'host0   /tmp/host_shared    9p      trans=virtio,version=9p2000.L   0 0' >> /etc/fstab && mount host0 && echo MOUNTEDDD")?;
    p.exp_string("MOUNTEDDD")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Successfully mounted host file share!");
    p.send_line("hostapd -B /tmp/host_shared/hostapd_wlan0_bravery_open.conf")?;
    p.exp_string("wlan0: AP-ENABLED")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Started first wifi radio!");
    p.send_line("hostapd -B /tmp/host_shared/hostapd_wlan1_cowardice_wpa.conf")?;
    p.exp_string("wlan1: AP-ENABLED")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Started second wifi radio!");
    p.send_line("/tmp/host_shared/ruwi wifi -i wlan2 connect -c netctl -e bravery")?;
    p.exp_string("[NOTE]: Successfully connected to: \"bravery\"")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Successfully connected to bravery with `-e`!");
    p.send_line("/tmp/host_shared/ruwi wifi -i wlan2 connect -c netctl -A known_or_fail")?;
    p.exp_string("[NOTE]: Successfully connected to: \"bravery\"")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Successfully connected to bravery with `-A known_or_fail`!");
    p.send_line("/tmp/host_shared/ruwi wifi -i wlan2 connect -c netctl -e cowardice -p cowardice2")?;
    p.exp_string("[NOTE]: Successfully connected to: \"cowardice\"")?;
    p.exp_string("@archiso")?;
    eprintln!("[TEST]: Successfully connected to cowardice with `-e` and `-p`!");
    // TODO: test scanning with iw
    // TODO: test refresh with iw by adding a new radio and network on it while ruwi is running?

    // fzf doesn't recognize non-control inputs sent with rexpect
    p.send_line("/tmp/host_shared/ruwi -m nocurses -i wlan2")?;
    p.exp_string("Select a network")?;
    eprintln!("[TEST]: Started ruwi in nocurses mode!");
    p.send_line("refresh")?;
    eprintln!("[TEST]: Sent refresh...");
    p.exp_string("[NOTE]: Refresh requested, running synchronous scan.")?;
    eprintln!("[TEST]: Refresh recognized...");
    p.exp_string("bravery")?;
    eprintln!("[TEST]: Refresh successful!");
    p.send_line("")?;
    // TODO: implement prefix matching
    // p.exp_string("[NOTE]: Successfully connected to: \"bravery\"")?;
    p.wait_for_prompt()?;


    eprintln!("[TEST]: Finished successfully!");
    eprintln!("[TEST]: Starting shutdown...");
    p.send_line("poweroff")?;
    eprintln!("[TEST]: Shutting down...");
    p.wait_for_prompt()?;
    eprintln!("[TEST]: Successfully shut down. Should exit immediately.");
    Ok(())
}
