# NEXUS Desktop Environment — Complete Project Plan
**Codename:** NEXUS-DE  
**Version:** 0.1 (Planning Phase)  
**Author:** Shyam Vadgama  
**Date:** September 2026  
**License (planned):** GPL-3.0-or-later  
**Repository:** github.com/Shyam-vadgama/nexus-de *(proposed)*

---

## 1. Vision & Problem Statement

Existing desktop environments fall into two camps:
- **Too minimal** (XFCE, i3, Sway) — niche audience, require heavy manual config, no beginner path
- **Too heavy/locked-in** (GNOME, KDE Plasma) — opinionated, hard to skin beyond themes, not Gen-Z aesthetic

**Hyprland** showed the market: millions of Gen-Z users will switch OSes just for aesthetics. But Hyprland is *only* a compositor — no shell, no settings, no out-of-box experience.

**NEXUS-DE** fills the gap: a *complete* desktop environment, Wayland-native, targeting Gen Z (18–28 yo) who want iOS/Android-level polish with Linux-level control. It ships in **three tiers** selectable at install time, so the same DE serves a student on a 4GB RAM laptop and a developer with a 4K setup.

---

## 2. Three-Tier Product Definition

### Tier 1 — NEXUS Minimal (`nexus-minimal`)
> Philosophy: "Fast, clean, still yours"

| Feature | Spec |
|---|---|
| Target | Old hardware, low RAM (2–4 GB), students |
| Compositor | Hyprland (hardware GPU accel, still light) |
| Animations | Subtle open/close, workspace slide (spring, <6ms) |
| Shell | Quickshell QML bar — minimal CPU footprint |
| Widgets | None on desktop by default |
| Glass effect | OFF |
| Wallpaper | Static only (swaybg) |
| RAM at idle | Target <180 MB |
| Config format | TOML via nexus-config daemon |

### Tier 2 — NEXUS Core (`nexus-core`)
> Philosophy: "Polished, productive, personalized"

| Feature | Spec |
|---|---|
| Target | Mid-range laptops, daily drivers |
| Compositor | Hyprland + blur plugin enabled |
| Animations | Spring physics — window open/close, workspace |
| Shell | Quickshell — dock + bar + notification center |
| Widgets | Drag-and-drop desktop widgets (clock, weather, calendar) |
| Glass effect | Subtle — panel + launcher only |
| Wallpaper | Static + animated (mpvpaper/swww) |
| App Launcher | NEXUS Launcher (custom, Material You colors) |
| Config | TOML + GUI Settings panel |
| RAM at idle | Target <280 MB |

### Tier 3 — NEXUS Hyper (`nexus-hyper`)
> Philosophy: "Full expression, zero compromise"

| Feature | Spec |
|---|---|
| Target | Modern hardware (16 GB+), power users, ricers |
| Compositor | Hyprland + full plugin stack (blur, rounded, shadows) |
| Animations | Full spring physics + shader-based transitions |
| Shell | Quickshell — full Material You shell |
| Widgets | Lock screen widgets + Home screen widgets (drag, resize, stack) |
| Glass effect | Full glassmorphism — windows, panels, widgets |
| Wallpaper | Live wallpaper engine (video, shader GLSL) |
| Lock screen | NEXUS Lock — widget-enabled, face/fingerprint |
| Control Center | Swipe-down panel (iOS style) |
| RAM at idle | Target <400 MB |

---

## 3. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      NEXUS-DE Stack                             │
├─────────────────────────────────────────────────────────────────┤
│  USER SPACE                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐   │
│  │ nexus-shell  │  │ nexus-widgets│  │ nexus-settings     │   │
│  │ (Quickshell/ │  │ (QML widget  │  │ (GTK4/libadwaita   │   │
│  │  Qt6/QML)   │  │  engine)     │  │  settings app)     │   │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬─────────┘   │
│         │                 │                       │             │
│  ┌──────▼─────────────────▼───────────────────────▼─────────┐  │
│  │                  NEXUS IPC Layer                          │  │
│  │     nexus-daemon (Rust) — D-Bus + Unix socket bridge      │  │
│  └──────────────────────┬────────────────────────────────────┘  │
│                         │                                        │
├─────────────────────────▼────────────────────────────────────────┤
│  COMPOSITOR LAYER                                               │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │           Hyprland (Wayland Compositor)                 │   │
│  │  + Aquamarine backend (DRM/KMS, libinput)               │   │
│  │  + hyprland-plugins (blur, rounded corners, etc.)       │   │
│  │  + xdg-desktop-portal-hyprland                         │   │
│  └──────────────────────────┬──────────────────────────────┘   │
│                             │                                    │
├─────────────────────────────▼────────────────────────────────────┤
│  SYSTEM SERVICES                                                │
│  PipeWire │ WirePlumber │ NetworkManager │ systemd              │
│  polkit   │ UPower      │ logind         │ D-Bus                │
├─────────────────────────────────────────────────────────────────┤
│  KERNEL / HARDWARE                                              │
│  DRM/KMS │ libinput │ Mesa/OpenGL │ Vulkan                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Tech Stack & Library Decisions

### 4.1 Compositor — Hyprland (C++)

**Decision: Hyprland, NOT a custom compositor**

Reason: Building a compositor from scratch (even wlroots-based) takes 2–3 years minimum before it's daily-drivable. Hyprland is already:
- Wayland-native, fully Aquamarine-backend (no wlroots dependency since v0.42)
- Plugin-based (Hyprland Plugin API in C++)
- IPC over Unix socket (`hyprctl` + `~/.config/hypr/hyprland.conf`)
- Actively maintained (v0.48+ in 2026, still fast moving)

**What we DO custom on top of Hyprland:**
- `nexus-hyprland.conf` — pre-configured base config per tier
- Custom Hyprland plugin: `nexus-effects-plugin` (C++) — shader-based glass, per-tier effects toggle
- `nexus-daemon` — wraps Hyprland IPC for higher-level DE operations

**Hyprland Plugin API targets:**
```cpp
// nexus-effects-plugin — hooks into Hyprland rendering pipeline
APICALL void hkRenderWindow(void* pWindow, ...) {
    // inject glass shader / frosted blur per tier
}
```

### 4.2 Shell Layer — Quickshell (Qt6/QML)

**Decision: Quickshell over AGS v2 / EWW**

| | Quickshell | AGS v2 (astal) |
|---|---|---|
| Language | QML/JavaScript | TypeScript/GJS |
| Rendering | Qt6 native (GPU) | GTK4 (CSS) |
| Glass blur support | ext-background-effect protocol | Limited |
| Performance | Excellent (Qt scenegraph) | Good |
| Widget system | Full QtQuick animations | CSS transitions |
| IPC | Any (socket, D-Bus, QProcess) | GLib-based |
| Maturity | Alpha but feature-rich (2026) | AGS v1 abandoned, v2 unstable |

Quickshell uses Qt6 + QML + C++ backend. It renders natively via Qt's Wayland integration and supports `ext-background-effect` for real blur behind panels.

**Shell components (all in Quickshell/QML):**
- `NexusBar` — top/bottom status bar, modular
- `NexusDock` — App dock (tier 2+), icon + running indicator
- `NexusLauncher` — App launcher, fuzzy search, Material You
- `NexusNotificationCenter` — notification center + history
- `NexusControlCenter` — swipe-down panel (tier 3)
- `NexusDesktopWidget` — draggable widget host
- `NexusLockscreen` — lock screen (tier 3, uses ext-session-lock)
- `NexusOSD` — volume/brightness overlay

### 4.3 Settings App — GTK4 + libadwaita (Python/Rust)

**Decision: GTK4 + libadwaita (NOT Qt) for settings**

Reason: GTK4/libadwaita gives beautiful, modern UI that matches GNOME HIG — users already know this pattern. Theming via CSS is simpler. Written in **Rust with gtk4-rs** crate for memory safety + performance.

**nexus-settings features:**
- Tier switcher (Minimal/Core/Hyper) with live preview
- Wallpaper picker + live preview
- Widget editor (drag preview)
- Color scheme / accent picker (Material You color extraction from wallpaper)
- Keybindings editor
- Per-app window rules
- Performance presets

### 4.4 Core Daemon — Rust (`nexus-daemon`)

The central IPC hub. All shell components talk to this daemon rather than directly to Hyprland. This makes shell components compositor-agnostic for future portability.

```
nexus-daemon
  ├── hyprland_ipc.rs    — reads Hyprland unix socket
  ├── dbus_server.rs     — exposes org.nexus.* D-Bus API
  ├── config_manager.rs  — reads/writes ~/.config/nexus/config.toml
  ├── theme_engine.rs    — Material You color computation
  ├── widget_registry.rs — tracks widget positions/state
  └── tier_manager.rs    — activates/deactivates feature sets per tier
```

**D-Bus API exposed (`org.nexus.DE`):**
- `SetTier(tier: u8)` — live switch between tiers
- `GetSystemInfo()` — CPU/RAM/network for widgets
- `NotifyWidget(id, data)` — push data to specific widget
- `SetWallpaper(path)` — trigger wallpaper change + recompute colors
- `GetAccentColors()` — Material You palette for current wallpaper

### 4.5 Widget Engine

Widgets are **QML components** registered with `nexus-daemon`. Each widget is a `.qml` file in `~/.config/nexus/widgets/` or `/usr/share/nexus/widgets/`.

**Widget manifest (JSON):**
```json
{
  "id": "nexus.clock",
  "name": "Clock",
  "min_tier": 2,
  "size": { "w": 200, "h": 100 },
  "resizable": true,
  "source": "clock.qml"
}
```

**Built-in widgets:**
- Clock (analog + digital)
- Date/Calendar
- Weather (Open-Meteo, no API key)
- System stats (CPU/RAM/disk — ring gauges)
- Media player (MPRIS2)
- Notes (quick sticky note)
- Network monitor
- Photo frame

**Drag-and-drop:** Handled by Quickshell's `DragHandler` + `DropArea` in QML, positions persisted to `~/.config/nexus/widget-layout.json` via nexus-daemon.

---

## 5. Complete Component Map

```
nexus-de/
├── nexus-compositor/          # Hyprland config templates + plugin
│   ├── configs/
│   │   ├── minimal.conf
│   │   ├── core.conf
│   │   └── hyper.conf
│   └── nexus-effects-plugin/  # C++ Hyprland plugin
│       ├── src/
│       │   ├── main.cpp
│       │   ├── GlassRenderer.cpp
│       │   └── AnimationOverrides.cpp
│       └── CMakeLists.txt
│
├── nexus-daemon/              # Rust IPC daemon
│   ├── src/
│   │   ├── main.rs
│   │   ├── hyprland_ipc.rs
│   │   ├── dbus_server.rs
│   │   ├── config_manager.rs
│   │   ├── theme_engine.rs    # Material You
│   │   ├── widget_registry.rs
│   │   └── tier_manager.rs
│   └── Cargo.toml
│
├── nexus-shell/               # Quickshell (QML/C++) shell
│   ├── bar/
│   │   ├── NexusBar.qml
│   │   ├── Workspaces.qml
│   │   ├── Clock.qml
│   │   ├── SystemTray.qml
│   │   └── MediaWidget.qml
│   ├── dock/
│   │   └── NexusDock.qml
│   ├── launcher/
│   │   └── NexusLauncher.qml
│   ├── notifications/
│   │   └── NexusNotifications.qml
│   ├── widgets/               # Desktop widget components
│   │   ├── WidgetHost.qml
│   │   ├── ClockWidget.qml
│   │   ├── WeatherWidget.qml
│   │   └── SystemWidget.qml
│   ├── lockscreen/
│   │   └── NexusLock.qml
│   ├── osd/
│   │   └── NexusOSD.qml
│   └── shell.qml             # entry point
│
├── nexus-settings/            # Rust + gtk4-rs settings app
│   ├── src/
│   │   ├── main.rs
│   │   ├── pages/
│   │   │   ├── appearance.rs
│   │   │   ├── wallpaper.rs
│   │   │   ├── widgets.rs
│   │   │   ├── keybindings.rs
│   │   │   └── about.rs
│   │   └── dbus_client.rs
│   └── Cargo.toml
│
├── nexus-greeter/             # Login screen (greetd + Quickshell)
│   └── Greeter.qml
│
├── nexus-wallpaper/           # Wallpaper daemon
│   ├── src/
│   │   ├── main.rs            # wraps swww / mpvpaper
│   │   └── shader_engine.rs   # GLSL live wallpaper (tier 3)
│   └── Cargo.toml
│
├── nexus-portal/              # xdg-desktop-portal backend
│   └── (fork of xdg-desktop-portal-hyprland, custom branding)
│
├── packages/                  # Distro packaging
│   ├── arch/
│   │   └── PKGBUILD
│   ├── debian/
│   │   └── control
│   └── fedora/
│       └── nexus-de.spec
│
└── docs/
    ├── ARCHITECTURE.md
    ├── CONTRIBUTING.md
    └── WIDGET_API.md
```

---

## 6. Build System & Language Decisions

| Component | Language | Build System | Reason |
|---|---|---|---|
| nexus-effects-plugin | C++ | CMake | Hyprland Plugin API is C++ only |
| nexus-daemon | Rust | Cargo | Memory safety, async D-Bus (zbus crate) |
| nexus-shell | QML + JS | qmake/CMake | Quickshell framework requirement |
| nexus-settings | Rust + gtk4-rs | Cargo | Memory safe, GTK4 native |
| nexus-wallpaper | Rust | Cargo | GLSL shader integration via glium/wgpu |
| nexus-greeter | QML | CMake | Consistent with shell |
| Hyprland configs | HyprLang | — | Hyprland native config |
| Widget manifests | JSON/QML | — | Runtime loaded |

---

## 7. Key Rust Crates

```toml
# nexus-daemon/Cargo.toml (key deps)
[dependencies]
zbus = "4"              # async D-Bus (pure Rust, replaces dbus-rs)
tokio = { features = ["full"] } # async runtime
hyprland = "0.4"        # Hyprland IPC Rust bindings
serde = { features = ["derive"] }
serde_json = "1"
toml = "0.8"            # config parsing
notify = "6"            # file watcher for config hot-reload
palette = "0.7"         # Material You color math (OkHSL)
image = "0.25"          # wallpaper color extraction

# nexus-settings/Cargo.toml
[dependencies]
gtk4 = "0.9"
libadwaita = "0.7"
zbus = "4"
```

---

## 8. QML/Shell Key Libraries

```
Qt 6.7+         — base runtime
QtQuick 2       — QML animation engine
QtWayland       — Wayland integration
Quickshell      — shell protocol implementations
  ├── WlrLayerShell    — panel/overlay surfaces
  ├── ExtSessionLock   — lockscreen protocol
  ├── WlrOutputManagement
  └── Hyprland IPC     — workspace/window info
```

---

## 9. UI Design System

### 9.1 Design Language — "Nexus Fluid"

Inspired by: iOS 17 Dynamic Island, Material You, Windows 11 Mica  
**NOT** copying: no flat GNOME, no KDE Breeze, no i3 minimal

**Core principles:**
1. **Motion is meaning** — every animation has purpose (open = grow from click point, close = shrink to dock)
2. **Depth through blur** — z-depth via frosted glass, not drop shadows
3. **Color from context** — Material You: accent extracted from wallpaper
4. **Responsive density** — UI adapts to tier, same design tokens

### 9.2 Design Tokens

```css
/* nexus-tokens.css / QML properties */
--radius-sm: 8px;
--radius-md: 14px;
--radius-lg: 22px;
--radius-xl: 30px;       /* widget cards */

--blur-subtle: 8px;      /* tier 2 panels */
--blur-medium: 20px;     /* tier 3 windows */
--blur-heavy: 40px;      /* lockscreen overlay */

--glass-alpha-low: 0.55;
--glass-alpha-mid: 0.72;

--anim-fast: 120ms;      /* OSD, tooltips */
--anim-mid: 280ms;       /* window open/close */
--anim-spring: cubic-bezier(0.34, 1.56, 0.64, 1); /* spring feel */

--shadow-dp1: 0 2px 8px rgba(0,0,0,0.18);
--shadow-dp3: 0 8px 24px rgba(0,0,0,0.28);
```

### 9.3 Color System — Material You (Dynamic)

`nexus-daemon`'s `theme_engine.rs` extracts dominant color from wallpaper using OkHSL colorspace, generates full Material You tonal palette.

Palette exported as:
- QML properties in Quickshell
- GTK CSS variables in nexus-settings
- Hyprland `col.active_border` / `col.inactive_border`

### 9.4 Animation Architecture

**Hyprland side (compositor):**
```
# nexus-hyper.conf
animations {
  enabled = true
  bezier = nexusSpring, 0.34, 1.56, 0.64, 1
  animation = windows, 1, 5, nexusSpring, popin 60%
  animation = windowsOut, 1, 4, default, popin 60%
  animation = workspaces, 1, 5, nexusSpring, slide
  animation = layers, 1, 3, nexusSpring, slide
}
```

**QML side (shell animations):**
```qml
// NexusBar.qml — smooth reveal
Behavior on opacity {
    NumberAnimation { duration: 200; easing.type: Easing.OutCubic }
}
SpringAnimation on y { spring: 2.5; damping: 0.4 }
```

---

## 10. Linux Integration Points

### 10.1 Session Management

**Login:** `greetd` + `nexus-greeter` (Quickshell QML)  
**Session start script:**
```bash
# /usr/share/wayland-sessions/nexus.desktop
[Desktop Entry]
Name=NEXUS Desktop
Exec=/usr/bin/nexus-session
```

```bash
# nexus-session script
#!/bin/bash
export XDG_CURRENT_DESKTOP=NEXUS
export XDG_SESSION_TYPE=wayland
export WAYLAND_DISPLAY=wayland-1

# Start daemon first
nexus-daemon &

# Import env for D-Bus/systemd
systemctl --user import-environment WAYLAND_DISPLAY XDG_CURRENT_DESKTOP
dbus-update-activation-environment --systemd WAYLAND_DISPLAY XDG_CURRENT_DESKTOP

# Start compositor (Hyprland reads tier config)
exec Hyprland -c ~/.config/nexus/hyprland-active.conf
```

### 10.2 IPC Flow Diagram

```
nexus-settings ──(D-Bus org.nexus.DE)──► nexus-daemon
                                               │
                              ┌────────────────┼────────────────┐
                              ▼                ▼                ▼
                       Hyprland IPC      ~/.config/nexus/   nexus-shell
                     (unix socket)       config.toml        (QML props)
                              │
                    hyprctl dispatch / keyword
```

### 10.3 D-Bus Services

```
org.nexus.DE          — nexus-daemon (main)
org.nexus.Wallpaper   — nexus-wallpaper
org.nexus.Widgets     — widget state management
org.nexus.Theme       — color system
```

### 10.4 XDG Integration

- `xdg-desktop-portal-hyprland` for screen share, file picker
- Follows XDG Base Dir spec: config in `~/.config/nexus/`, data in `~/.local/share/nexus/`
- `XDG_CURRENT_DESKTOP=NEXUS` for app detection
- `.desktop` files in `/usr/share/applications/nexus-*.desktop`

### 10.5 Systemd User Units

```ini
# /usr/lib/systemd/user/nexus-daemon.service
[Unit]
Description=NEXUS Desktop Daemon
PartOf=graphical-session.target

[Service]
ExecStart=/usr/bin/nexus-daemon
Restart=on-failure

[Install]
WantedBy=graphical-session.target
```

---

## 11. Opensource Reusable Components (Don't Reinvent)

| Need | Reuse | License |
|---|---|---|
| Compositor | Hyprland (configure + plugin) | BSD-3 |
| Shell framework | Quickshell | LGPL-3 |
| Wayland protocols | wayland-protocols, hyprland-protocols | MIT |
| Audio | PipeWire + WirePlumber | MIT/LGPL |
| Notifications | Quickshell built-in | LGPL |
| Wallpaper engine | swww (Rust, animated wallpaper) | GPL-3 |
| Live wallpaper | mpvpaper (video wallpaper) | GPL-2 |
| Screenshot | grim + slurp | MIT |
| App launcher base | fuzzel (fork for styling) | MIT |
| System info | sysinfo Rust crate | MIT |
| Weather API | Open-Meteo (free, no key) | CC-BY-4 |
| Material You | material-color-utilities (Rust port) | Apache-2 |
| Color extraction | palette Rust crate | MIT |
| Font | Inter (UI), JetBrains Mono (terminal) | OFL |
| Icon theme | Papirus (default, forkable) | GPL-3 |
| GTK theme | adw-gtk3 (for GTK3 app compat) | LGPL |
| D-Bus Rust | zbus | MIT |
| Login manager | greetd | GPL-3 |

---

## 12. Phase-by-Phase Roadmap

### Phase 0 — Research & Skeleton (Month 1–2)
- [ ] Finalize this plan, freeze scope
- [ ] Setup Hyprland plugin dev environment
- [ ] Hello-world Quickshell config running
- [ ] nexus-daemon skeleton (D-Bus ping, config read)
- [ ] GitHub repo init with proper README, CONTRIBUTING.md
- [ ] Write ARCHITECTURE.md (this doc expanded)

### Phase 1 — Core Plumbing (Month 3–5)
- [ ] nexus-daemon: Hyprland IPC wrapper working
- [ ] nexus-daemon: D-Bus server live
- [ ] nexus-daemon: config.toml read/write + hot reload
- [ ] nexus-shell: NexusBar basic (time, workspaces, tray)
- [ ] nexus-shell: NexusLauncher basic (app search)
- [ ] Tier 1 (Minimal) fully functional
- [ ] nexus-session script working on Arch

### Phase 2 — Shell Polish + Tier 2 (Month 6–9)
- [ ] NexusDock with running indicators
- [ ] NexusNotifications with center
- [ ] Desktop widget engine (drag/drop working)
- [ ] Built-in widgets: Clock, Weather, System
- [ ] nexus-settings app: appearance + wallpaper pages
- [ ] Material You color engine integrated
- [ ] Tier 2 (Core) fully functional
- [ ] AUR PKGBUILD published

### Phase 3 — Hyper Tier + Glass (Month 10–13)
- [ ] nexus-effects-plugin: glass shader for Hyprland
- [ ] NexusLockscreen with widget support
- [ ] NexusControlCenter (swipe panel)
- [ ] Live wallpaper support (mpvpaper / GLSL shaders)
- [ ] Lockscreen widgets (clock, weather, media)
- [ ] Tier 3 (Hyper) fully functional
- [ ] nexus-greeter for login screen

### Phase 4 — Hardening & Community (Month 14–18)
- [ ] Fedora RPM + Debian package
- [ ] Multi-monitor tested
- [ ] NVIDIA explicit-sync tested
- [ ] Performance profiling (target: 60 fps constant, <400MB RAM tier 3)
- [ ] Widget SDK documented (WIDGET_API.md)
- [ ] Theme SDK documented
- [ ] First community contributions merged

---

## 13. Open Questions / Design Decisions Still Pending

1. **IPC protocol v2** — should nexus-daemon expose a REST/HTTP API locally (easier for future Electron-based settings)?
2. **Widget sandboxing** — third-party widgets from community: run in separate process, or trust model like GNOME extensions?
3. **Wayland compositor portability** — Phase 4: can nexus-shell also work on niri/sway, or stay Hyprland-only? (Quickshell makes this possible, but IPC adapters needed)
4. **Mobile/convergent path** — Phosh/Plasma Mobile exists; should NEXUS-DE have a phone tier someday?
5. **Lock screen security** — `ext-session-lock` protocol is compositor-enforced; need security audit before shipping
6. **Config migration** — when user switches tiers, how much of their widget layout carries over?

---

## 14. Git Repository Structure

```
github.com/Shyam-vadgama/nexus-de  (monorepo)
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── workflows/
│       ├── build.yml      # CI: cargo build + cmake build
│       └── lint.yml       # clippy + clang-tidy
├── README.md              # screenshots, install, tier comparison
├── CONTRIBUTING.md
├── ARCHITECTURE.md
├── SECURITY.md
└── [component dirs as above]
```

---

## 15. Success Metrics

| Metric | Target |
|---|---|
| Idle RAM — Tier 1 | < 180 MB |
| Idle RAM — Tier 3 | < 400 MB |
| App launch animation | < 300ms visible |
| Config hot-reload | < 100ms |
| First-install to working desktop | < 5 commands |
| Widget drag latency | < 16ms (60 fps) |
| GitHub stars (6 months post-launch) | > 500 |
| Supported distros at v1.0 | Arch, Fedora, Debian/Ubuntu |

---

*This document is the single source of truth for NEXUS-DE architecture decisions.*  
*Last updated: September 2026*  
*Next review: After Phase 0 completion*
