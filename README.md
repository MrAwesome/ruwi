# ruwi

Ruwi simplifies connecting to the Internet on Linux.

It does this by providing a simple CLI for interacting with many different existing Linux networking and selection utilities.

Bluetooth support is planned, but currently not in progress.

Ruwi is still heavily under construction, but feel free to tinker away. Wired and wireless connections are thoroughly tested and working well. If something doesn't work for you, feel free to create an issue on GitHub.

Ruwi is written in 100% safe Rust, and will stay that way.

# Strengths and Weaknesses

### Some things Ruwi can do well:
* Provide a human-intuitive CLI for connecting to the Internet (`ruwi wired connect` vs. `dhcpcd enp0s25`)
* Present available networks for selection in the least amount of time and with the most convenience possible
* Connect to known networks, *regardless of configuration complexity*, by offloading the work to netctl/NetworkManager
* Prioritize seen wifi networks based on signal strength and whether or not that network is already known
* Use that prioritization to automatically connect to the strongest known network, if requested
* Decide whether or not a synchronous wifi rescan is needed based on the results of a cached scan
* Support many different functional combinations of scanning/selection/connection methods
* Handle bringing networking interfaces and services up or down when needed
* Discover and connect to new non-encrypted or WPA2 (password) wifi networks
* Discover and connect on new non-encrypted Ethernet interfaces
* Piggyback on existing battle-tested scanning, selection, and connection technologies
* Detect which connection managers and scanning utilities are installed/running, and make intelligent decisions about which ones to use
* Support keyboard-centric workflows, both at the CLI and via keybindings in window managers

### Some things Ruwi does not do:
* Communicate directly with the kernel to facilitate scanning/connection<sup id="a3">[3](#f3)</sup>
* Add configuration details for wifi connections beyond "here's the encryption key (password)"
* Add configuration details for encrypted/complex Ethernet connections
* Support mouse-centric workflows (although fzf does have support for clicking, if you're using that you might be missing the point)

# Philosophy

### Ruwi is:
* Fast (from a computational point of view). Generally, a dryrun of Ruwi will take less than one hundredth of a second. In normal runs you will still need to wait on the networking stack, but Ruwi gets itself out of the way blazingly fast.
* Fast (from a human interaction point of view). Ruwi will never ask for a non-essential piece of information from you. By relying on well-designed and battle-tested selection programs such as `fzf` and `dmenu`, Ruwi can help you connect to a new wireless network faster than any other program on the planet.<sup id="a1">[1](#f1)</sup>
* Keyboard-driven. Ruwi is designed to be run in two ways: from a terminal, or bound to a hotkey in your window manager. You'll never need to click anything in Ruwi.
* Written in 100% safe Rust.
* Designed to be safe to run as root. External programs will fail to run if they are not binaries with full paths owned and writable only by the root user.
* Well-tested. All core functionality and common workflows are unit and integration tested, and new code will not be committed until all tests pass.
* Clean. Barring occasional intra-codemod commits, all code passes pedantic-level `clippy` linting, and will stay that way.
* Smart. Ruwi tries to infer the best programs to use based on what's currently installed and/or running on your system.

### Ruwi is not:
* A connection manager. There is no daemon process, there are no networking configs. Ruwi works entirely by orchestrating external programs, including connection managers. For example, you can scan with `iw` or `nmcli`, select with `fzf` or `dmenu`, and connect with `netctl` or `nmcli`<sup id="a2">[2](#f2)</sup>.
* A connection or scanning utility. Ruwi tries to know as little as possible about networking, and tries instead to use external programs and libraries to offload all interactions with the kernel and networking devices.
* Stateful. Ruwi remembers nothing about previous runs, remembers nothing about individual networks. What you see on the command line is what you get. Any state relating to a network (whether it is already known, the encryption key, etc) is stored with the service used to connect to it, such as `netctl` or `NetworkManager`.
* Designed to handle complicated network configuration. Anything more complex than "use this WPA2 passphrase" is not supported, by design. With that said, you only need to set up your complex config once in your connection manager (netctl config file, NetworkManager network, etc) and Ruwi will detect it and happily help you connect to it quickly from that point on.


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

<b id="f1">1</b> In truth, it's hard to imagine any "find, select, and connect to an unknown wireless network" workflow being faster than what Ruwi does. If you know of something, contact your dear author immediately. [↩](#a1)

<b id="f2">2</b> With the caveat that `nmcli` doesn't play nicely with other scan types. This is handled in code, and a descriptive error message will guide you if you forget. [↩](#a2)

<b id="f3">3</b> There's no reason Ruwi can't do these things, but it would make much more sense to add the functionality into another crate and simply plug that into Ruwi. [↩](#a3)
