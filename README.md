# :rocket: upwork-wlroots-bridge :rocket:

Rust Implementation for Wlroots (Sway, Wayfire, Hikari, River, etc.) of Gnome
Screenshot and Idle DBUS Server (with extra features), which Upwork uses to capture the screen as
proof of work.

This work was inspired by the python implementation of the Gnome Screenshot DBUS
Server made by MarSoft, which can be found [here](https://github.com/MarSoft/upwork-wayland).

## :hourglass_flowing_sand: Quick Start

this section will cover the minimum amount of information to get you up and
running.

### :computer: Installation

First, you must install the required packages from your distribution package
manager. These packages are:

- [Rust](https://www.rust-lang.org/) => to use cargo.
- [Grim](https://sr.ht/~emersion/grim/) => screenshot tool used.
- [Swayidle](https://github.com/swaywm/swayidle) => to calculate idle time.

##### Arch Linux

```
sudo pacman -S swayidle grim rust
```

##### Debian, Ubuntu & Derivatives

```
sudo apt install swayidle grim rust-all
```

##### Fedora & Derivatives

```
sudo dnf install swayidle grim rust
```

#### Cargo install

Right now, the more straightforward method of installation is via cargo with
the command:

```
cargo install upwork-wlroots-bridge
```

### :pencil2: Usage

You can use `upwork-wlroots-bridge` with no options at all and launch it as is. 
In this mode, you will receive no feedback, but that's it. 
You can now use Upwork desktop client as normal once you have launched the server. 
```
upwork-wlroots-bridge
```

Besides that, `upwork-wlroots-bridge` comes with some valuable flags like `-w`, 
which will present a warning dialog and sound through zenity, and pw-play when 
a screenshot is incoming (so you can refocus your work and not be surprised by 
those annoying "surprise" screenshots).
```
upwork-wlroots-bridge -wD   # The D stands for Debug, which will present some more info
```

When you are done choosing your ideal flags, then you can daemonize the server in 
your preferred way ([tmux](https://github.com/tmux/tmux/wiki), [systemd services](https://wiki.archlinux.org/title/systemd), [nohup](https://low-orbit.net/linux-how-to-nohup)).
