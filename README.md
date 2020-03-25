# `ruwi` - Connection Selection for Desktop Linux

Ruwi simplifies connecting to the Internet on Linux.

It does this by providing a flexible+extensible selection+interaction layer over many different Linux networking utilities. Bluetooth support is planned. 

Ruwi is still heavily under construction, but feel free to tinker away. Wired and wireless connections are heavily tested and working well. It's very unlikely that the commands below will change, and for these use cases ruwi makes an excellent daily driver.

# Philosophy

### Ruwi is:
* Fast (from a computational point of view). Generally, a dryrun of Ruwi will take less than one hundredth of a second. Ruwi cannot speed up the process of scanning and connecting to networks, but it can certainly speed up everything in between.
* Fast (from a human interaction point of view). Ruwi will never ask for a non-essential piece of information from you. By relying on well-designed and battle-tested selection programs such as `fzf` and `dmenu`, Ruwi can help you connect to a new wireless network faster than any other program on the planet.
* Keyboard-driven. Ruwi is designed to be run in two ways: from a terminal, or bound to a hotkey in your window manager. You'll never need to click anything in Ruwi.
* Written in 100% safe Rust.
* Designed to be safe to run as root. External programs will fail to run if they are not binaries with full paths owned and writable only by the root user.
* Well-tested. All core functionality and common workflows are unit and integration tested, and new code will not be committed until all tests pass.
* Clean. Barring occasional intra-codemod commits, all code passes pedantic-level `clippy` linting, and will stay that way.

### Ruwi is not:
* A connection manager. There is no daemon process, there are no networking configs. Ruwi works entirely by orchestrating external programs, including connection managers. For example, you can scan with `iw` or `nmcli`, select with `fzf` or `dmenu`, and connect with `netctl` or `nmcli`<sup id="a1">[1](#f1)</sup>.
* A connection or scanning utility. Ruwi tries to know as little as possible about networking, and tries instead to use external programs and libraries to offload all interactions with the kernel and networking devices.
* Stateful. Ruwi remembers nothing about previous runs, remembers nothing about individual networks. What you see on the command line is what you get.

# Manual Installation
While still under construction, ruwi will not live in any distro repositories, and you'll need to build it for yourself using `cargo`. See https://www.rust-lang.org/tools/install if you aren't sure what `cargo` is.

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

---

In truth, it's hard to imagine any "find, select, and connect to an unknown wireless network"

<b id="f1">1</b> With the caveat that `nmcli` doesn't play nicely with other scan types. This is handled in code, and a descriptive error message will guide you if you forget. [↩](#a1)
