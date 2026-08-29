# CASDE

[![CI](https://github.com/casapps/casde/actions/workflows/ci.yml/badge.svg)](https://github.com/casapps/casde/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/casapps/casde)](https://github.com/casapps/casde/releases)
[![License](https://img.shields.io/github/license/casapps/casde)](LICENSE.md)

## About

CASDE is a full desktop environment for Linux (and BSD, opt-in — see
`IDEA.md`) with a built-in, feature-rich compositor. It replaces the
fragmented WM/DE landscape (i3, Sway, BSPWM, Awesome, XFCE4, and the
rest) with one coherent environment: compositor, panel/taskbar, app
menu, window management, keybindings, theming, and system integration
all ship in a single static binary.

XFCE4's own configuration is the accuracy baseline and default — see
`IDEA.md` for the full feature/decision record.

## Features

- Native Wayland compositor and native X11 window manager, both with
  full effect parity (no second-class backend)
- Multi-tag virtual desktops/workspaces, 9 default workspaces
  (dev/web/sys/office/virt/comm/audio/video/gfx)
- Built-in panel/taskbar/app menu with a fixed widget catalog
- Super-based keybindings plus XF86 media keys
- YAML config with a full settings UI (`casdectl` + GUI)
- Full multi-monitor support and accessibility from day one
- Screenshot tool, clipboard manager, PipeWire/PulseAudio integration,
  a notification daemon (`org.freedesktop.Notifications`), a native
  keyring (full Secret Service API), and an `xdg-desktop-portal` compat
  provider
- 50 bundled themes (25 dark + 25 light), Hack Nerd Font default, Noto
  Color Emoji

This is implementation work in progress — see `TODO.AI.md` for the
current backlog.

## Installation

Download the static binary for your platform from
[Releases](https://github.com/casapps/casde/releases) and run it — no
separate runtime dependencies beyond the kernel and a display socket.

## Usage

```bash
casde                 # auto-detects GUI vs CLI (see "Mode behavior" below)
casde --ui gui        # force the DE session
casde --ui cli        # force the casdectl-style CLI
casde --licenses       # print embedded third-party licenses
casde servers          # list running CASDE servers over the IPC socket
```

## Mode behavior (GUI / CLI)

CASDE has two surfaces (no TUI — a desktop environment has no meaningful
terminal-only mode, see `IDEA.md` "App surfaces"):

- **GUI** — the DE session itself: compositor, panel, window management.
  Selected automatically when a display socket is present and the
  process is not in a remote shell. Both `WAYLAND_DISPLAY` (preferred)
  and `DISPLAY` (X11/XWayland fallback) are supported as first-class
  backends — see AI.md PART 0 "X11 AND Wayland Are Both Required".
- **CLI** (`casdectl`-style) — automation fallback that talks to the
  running session over its IPC socket at
  `$XDG_RUNTIME_DIR/casde/casde.sock`.

## Configuration

Config lives at `~/.config/casapps/casde/config.yaml` (XDG-standard,
frozen on the `casapps`/`casde` identifier pair — see AI.md PART 4
"Path Rule"). The app runs with full default functionality when no
config file is present.

## Development

The Rust toolchain never runs on the host — every command runs inside
Docker (`casjaysdev/rust:latest`). See `docker/README.md` for the full
reference; the short version:

```bash
make fmt-check
make lint
make test
make build-release
```

## Testing

```bash
make test                 # cargo test inside the toolchain container
make docker-test           # docker-compose.test.yml, containerized
make run-gui                # smoke-test the GUI with X11/Wayland forwarding
```

GUI tests exercise both X11 and Wayland forwarding — see
`docker/README.md` "Run the GUI with display forwarding".

## License

MIT — see [LICENSE.md](LICENSE.md) for the full text and third-party
attributions.
