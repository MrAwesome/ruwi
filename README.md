# ruwi

Ruwi simplifies connecting to the Internet on Linux, by providing a flexible selection and interaction layer over many different Linux networking utilities. Bluetooth support is planned. 

Ruwi is still heavily under construction, but feel free to tinker away. Wired and wireless connections are heavily tested and working well. It's very unlikely that the commands below will change, and for these use cases ruwi makes an excellent daily driver.

# Manual Installation
while still under construction, ruwi will not live in any distro repositories, and you'll need to build it for yourself using `cargo`. See https://www.rust-lang.org/tools/install if you aren't sure what `cargo` is.

    cd ~/
    git clone https://github.com/MrAwesome/ruwi.git
    cd ruwi/
    cargo build
    
After this, you can run `ruwi` directly:

    sudo ~/ruwi/target/debug/ruwi 
    
For convenience, you'll probably want to add this alias to your `.bashrc` or `.zshrc` or etc:
    alias ruwi=`sudo ~/ruwi/target/debug/ruwi`

# Usage
NOTE: Ruwi should almost always be run using `sudo`. This assumes you've aliased it as shown above.

Scan for wifi networks, ask the user to select a network, and connect to the selected network: 

    ruwi wifi connect

Scan for wifi networks, automatically select the strongest known network (or ask the user to select a network, if no known networks are seen), and connect to it:

    ruwi wifi connect -a

Connect on the first wired interface seen on the system:

    ruwi wired connect

Connect on the named wired interface, using `dhclient`:

    ruwi wired -i "enp111s0" connect -c "dhclient"
    
Stop all known networking daemons, bring down all IP networking interfaces, and just generally get a clean slate for attempting to connect:

    ruwi clear

# Philosophy

## Ruwi is:
* Designed to be safe to run as root. External programs will fail to run if they are not binaries with full paths owned and writable only by the root user.

## Ruwi is not:
* A connection manager. There is no daemon process, there are no networking configs. Ruwi works entirely by orchestrating external programs, including connection managers. For example, you can scan with `iw` or `nmcli`, select with `fzf` or `dmenu`, and connect with `netctl` or `nmcli`<sup id="a1">[1](#f1)</sup>.


---

<b id="f1">1</b> With the caveat that `nmcli` doesn't play nicely with other scan types. This is handled in code, and a descriptive error message will guide you if you forget. [↩](#a1)
