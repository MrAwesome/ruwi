use rexpect::errors::*;
use rexpect::spawn_bash;

#[test]
#[ignore]
fn test_full_integration_with_archlinux_vm() -> Result<()> {
    let mut download_iso_shell = spawn_bash(Some(900000))?;

    // TODO: Other targets as well
    download_iso_shell.execute(
        "mkdir -p /tmp/archlinux/shared && cp -r ./target/debug/ruwi ./ci/configs/*.conf /tmp/archlinux/shared && echo COPIEDDD",
        "COPIEDDD",
    )?;
    download_iso_shell.wait_for_prompt()?;

    let mut p = spawn_bash(Some(32000))?;
    let command = concat!(
        "cd /tmp/archlinux/ && ",
        "curl http://mirror.rackspace.com/archlinux/iso/2019.12.01/md5sums.txt | grep archlinux-2019.12.01-x86_64.iso | md5sum -c || ",
        "curl -O http://mirror.rackspace.com/archlinux/iso/2019.12.01/archlinux-2019.12.01-x86_64.iso && ",
        "curl http://mirror.rackspace.com/archlinux/iso/2019.12.01/md5sums.txt | grep archlinux-2019.12.01-x86_64.iso | md5sum -c && ",
        "echo DOWNLOADEDBRAH || ",
        "exit 1"
    );

    p.execute(&command, "DOWNLOADEDBRAH")?;
    println!("[TEST]: Successfully downloaded/checksummed LiveCD!");
    p.wait_for_prompt()?;
    p.execute(
        "qemu-system-x86_64 -cdrom /tmp/archlinux/archlinux-2019.12.01-x86_64.iso -m 1024 -enable-kvm -nographic -vga none -virtfs local,path=/tmp/archlinux/shared,mount_tag=host0,security_model=passthrough,id=host0", 
        "to edit options"
    )?;
    println!("[TEST]: Reached kernel options...");
    p.send_control('i')?;
    p.exp_string("vmlinuz")?;
    p.send_line(" console=ttyS0")?;
    println!("[TEST]: Started boot...");

    p.exp_string("archiso login:")?;
    println!("[TEST]: Successfully reached prompt!");

    p.send_line("root")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Successfully logged in!");

    println!("[TEST]: Installing packages...");
    p.send_line("pacman -Sy --noconfirm hostapd && echo INSTALLEDITYES")?;
    p.exp_string("INSTALLEDITYES")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Successfully installed packages!");
    p.send_line("modprobe mac80211_hwsim radios=3 && echo MODPROBED")?;
    p.exp_string("MODPROBED")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Successfully loaded virtual radios!");
    p.send_line("mkdir -p /tmp/host_shared && echo 'host0   /tmp/host_shared    9p      trans=virtio,version=9p2000.L   0 0' >> /etc/fstab && mount host0 && echo MOUNTEDDD")?;
    p.exp_string("MOUNTEDDD")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Successfully mounted host file share!");
    p.send_line("hostapd -B /tmp/host_shared/hostapd_wlan0_bravery_open.conf")?;
    p.exp_string("wlan0: AP-ENABLED")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Started first wifi radio!");
    p.send_line("hostapd -B /tmp/host_shared/hostapd_wlan1_cowardice_wpa.conf")?;
    p.exp_string("wlan1: AP-ENABLED")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Started second wifi radio!");
    p.send_line("/tmp/host_shared/ruwi -i wlan2 -e bravery")?;
    p.exp_string("[NOTE]: Successfully connected to: \"bravery\"")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Successfully connected to bravery with `-e`!");
    p.send_line("/tmp/host_shared/ruwi -i wlan2 -A known_or_fail")?;
    p.exp_string("[NOTE]: Successfully connected to: \"bravery\"")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Successfully connected to bravery with `-A known_or_fail`!");
    p.send_line("/tmp/host_shared/ruwi -i wlan2 -e cowardice -p cowardice2")?;
    p.exp_string("[NOTE]: Successfully connected to: \"cowardice\"")?;
    p.exp_string("@archiso")?;
    println!("[TEST]: Successfully connected to cowardice with `-e` and `-p`!");
    //    p.send_line("/tmp/host_shared/ruwi -i wlan2")?;
    //    p.exp_string("Select a network")?;
    //    println!("[TEST]: Started ruwi in fzf mode!");
    //    p.send_control('r')?;
    //    println!("[TEST]: Sent refresh...");
    //    p.exp_string("[NOTE]: Refresh requested, running synchronous scan.")?;
    //    println!("[TEST]: Refresh recognized...");
    //    p.exp_string("bravery")?;
    //    println!("[TEST]: Refresh successful!");
    //    // Make sure we aren't just selecting the first option
    //    p.send_control('n')?;
    //    p.send_control('n')?;
    //    p.send_control('n')?;
    //    p.send_control('v')?;
    //    p.send("b")?;
    //    p.flush()?;
    //    p.send_control('v')?;
    //    p.send("r")?;
    //    p.flush()?;
    //    p.send_control('v')?;
    //    p.send("a")?;
    //    p.flush()?;
    //    p.send("\n")?;
    //    p.flush()?;
    //    p.exp_string("[NOTE]: Successfully connected to: \"bravery\"")?;
    //    println!("[TEST]: Successfully connected to bravery!");
    Ok(())
}
