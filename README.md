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

- Rust => to use cargo.
- Grim => screenshot tool used.
- Swayidle => to calculate idle time.

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
