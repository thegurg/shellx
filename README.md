# shellx / Amane Shell

A personal [Amane](https://github.com/MystiaFin/amane)-based desktop shell for **Niri** and **MangoWC** (Niri-first), written in Rust.

It provides a Gruvbox-styled bar, launcher, control center, notification popups/center, system monitor window, lock screen, and a left quick-action rail - all theme-aware and built as one Amane config.

## Features

- **Top bar** - workspaces, active window, clock, quick status, control center / rail entry points.
- **Launcher** - built-in overlay launcher with search, keyboard navigation, selection-follow scroll, click-outside close.
- **Control center** - unified overlay with audio, brightness, network, bluetooth, media, battery, and power actions.
- **Notifications** - bottom-right popups (auto-hide after 3s) + notification center panel from the rail. Uses Amane's `org.freedesktop.Notifications` service.
- **Lock screen** - Amane/PAM lock screen with Gruvbox styling and theme-aware visuals.
- **Left rail** - autohide hover rail for Launcher, Notifications, Monitor, Theme cycle, Variant, and Waypaper.
- **Monitor window** - normal window with CPU, RAM, and filesystem usage.
- **Themes/variants** - Gruvbox: `Ember` (warm/rounded), `Chroma` (sharp/colorful). Everforest: `Mist` (minimal), `Bloom` (colorful). Switchable at runtime.
- **Config/settings** - simple `~/.config/amane/config.conf`, `config`/`settings` IPC, and a basic settings window.

## Requirements

- Linux + Wayland compositor (tested direction: Niri / MangoWC)
- [Amane CLI](https://github.com/MystiaFin/amane) installed (`amane dev`, `amane run`, `amane ipc`)
- Rust toolchain / Cargo (for Amane dev/build flow)
- JetBrainsMono Nerd Font (configured in `src/main.rs`)
- Optional/external tools used by integrations:
  - `waypaper` for wallpaper picking
  - `hypridle` for idle management (current setup)
  - `brightnessctl`, `wpctl`/PipeWire, NetworkManager, bluetooth stack depending on your system
  - `notify-send` for notification testing

> Note: `Cargo.toml` currently points to a local Amane library path (`/home/persius/.cache/amane/library`). Adjust it for your local Amane checkout/install layout.

## Development

From the Amane config directory:

```sh
amane dev
```

Run/compile:

```sh
amane compile
amane run
```

## Config

`~/.config/amane/config.conf` (simple key=value):

```conf
# Amane shell config
theme = "chroma"
notification_timeout_secs = 3
bar_enabled = true
rail_enabled = true
notifications_enabled = true
monitor_enabled = true
lock_enabled = true
```

Module toggles apply on restart; theme/timeout can be applied live via IPC/settings.

## IPC

```sh
amane ipc call launcher toggle|open|close|status
amane ipc call control-center toggle|open|close|status
amane ipc call rail toggle|show|hide|status
amane ipc call notifications toggle|open|close|clear|status
amane ipc call monitor toggle|open|close
amane ipc call theme current|list|cycle|ember|chroma|mist|bloom
amane ipc call config get|reload|settings
amane ipc call settings open|close
amane ipc call lock
```

## MangoWC keybinds (example)

```conf
bind=SUPER,r,spawn_shell,amane ipc call launcher toggle
bind=SUPER+SHIFT,space,spawn_shell,amane ipc call control-center toggle
bind=SUPER+ALT,b,spawn_shell,amane ipc call rail toggle
bind=SUPER+ALT,t,spawn_shell,amane ipc call theme cycle
```

## Project structure

```text
.
├── Cargo.toml
├── shaders/
│   └── grain.wgsl
└── src/
    ├── main.rs              # App wiring, windows, IPC
    ├── bar.rs               # Top bar
    ├── launcher.rs          # Launcher overlay
    ├── control_center.rs    # Control center overlay
    ├── notifications.rs     # Popups + notification center
    ├── monitor.rs           # CPU/RAM/filesystem window
    ├── lock_screen.rs       # Lock screen
    ├── rail.rs              # Left autohide rail
    ├── active_window.rs     # Focused window/title helpers
    ├── clock.rs             # Clock service
    └── theme/               # Gruvbox theme variants
```

## Notes

- **Notifications**: only active if no other notification daemon owns `org.freedesktop.Notifications`.
- **Idle**: idle behavior is configured externally (currently `hypridle` in my setup); adjust for your compositor/environment.
- **Waypaper**: wallpaper management is intentionally external (`waypaper`/`awww`) after trying a native manager.

## License

MIT (see `LICENSE` if present, or treat as MIT unless changed).
