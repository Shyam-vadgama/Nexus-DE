# NEXUS-DE Architecture Specification

**Codename:** NEXUS-DE  
**Author:** Shyam Vadgama  
**Status:** In Development  

---

## 1. High-Level Architecture

NEXUS-DE separates concerns between the low-level Wayland compositor, a central Rust-based background state and IPC daemon, and a modular Qt6/QML desktop shell.

```
┌─────────────────────────────────────────────────────────────┐
│                      NEXUS-DE Shell                         │
│   Quickshell (Qt6/QML) — Bar, Dock, Launcher, OSD, Widgets  │
└──────────────────────────────┬──────────────────────────────┘
                               │ (D-Bus: org.nexus.DE)
┌──────────────────────────────▼──────────────────────────────┐
│                       nexus-daemon                          │
│   • Hyprland IPC bridge (Unix sockets)                      │
│   • D-Bus server & client coordination                      │
│   • Dynamic Material You palette extraction (OkHSL)         │
│   • Configuration manager (~/.config/nexus/config.toml)     │
│   • Live Tier management (Minimal / Core / Hyper)           │
└──────────────────────────────┬──────────────────────────────┘
                               │ (Unix socket: .socket.sock / .socket2.sock)
┌──────────────────────────────▼──────────────────────────────┐
│                    Hyprland Compositor                      │
│   • Aquamarine backend (DRM/KMS, libinput)                  │
│   • Layer-shell rendering                                   │
│   • Optional C++ effects plugin (Tier 3 progressive)        │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Component Directory Structure

- **`crates/nexus-core`**: Core domain logic, config schema (`NexusConfig`), tier definition (`NexusTier`), and Material You palette generator. Zero external UI dependencies; fully unit-tested.
- **`crates/nexus-ipc`**: Hyprland IPC client and D-Bus interfaces.
- **`apps/nexus-daemon`**: Background system service exposing `org.nexus.DE` on the user session bus.
- **`apps/nexus-settings`**: Modern GTK4 + libadwaita preferences application.
- **`shell/`**: Quickshell (QML) desktop environment components (Bar, Dock, Launcher, OSD, Widgets, Lockscreen).
- **`compositor/`**: Hyprland configuration presets (`minimal.conf`, `core.conf`, `hyper.conf`) and optional C++ effects plugin.
- **`session/`**: Session launch scripts (`nexus-session`), Wayland desktop entries, and systemd user services.
- **`packaging/`**: Native distribution packaging (Arch PKGBUILD, Fedora spec).

---

## 3. Communication Protocols

1. **Hyprland to Daemon**: Communicates over `$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket.sock` for commands (dispatch, get active window) and `.socket2.sock` for event streams (workspace switch, window focus, monitor connect).
2. **Daemon to Shell**: Communicates via D-Bus session bus (`org.nexus.DE`). The daemon emits signals on state transitions (`TierChanged`, `ThemeChanged`, `MetricsUpdated`), which Quickshell components react to reactively.
3. **Settings App to Daemon**: Direct method calls over D-Bus (`SetTier`, `SetWallpaper`, `GetAccentColors`).
