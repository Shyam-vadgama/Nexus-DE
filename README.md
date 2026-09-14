# NEXUS Desktop Environment (NEXUS-DE)

[![CI](https://github.com/Shyam-vadgama/nexus-de/actions/workflows/ci.yml/badge.svg)](https://github.com/Shyam-vadgama/nexus-de/actions)
[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](LICENSE)
[![Wayland Native](https://img.shields.io/badge/Wayland-Native-brightgreen.svg)]()
[![Hyprland Powered](https://img.shields.io/badge/Hyprland-0.56%2B-00ADD8.svg)]()

**NEXUS-DE** is a modern, modular Wayland desktop environment built for Gen-Z power users who want iOS/Android-level polish paired with full Linux-level control.

Built natively on **Hyprland**, **Quickshell (Qt6/QML)**, and **Rust**, NEXUS-DE introduces an adaptive **Three-Tier Architecture** that delivers consistent aesthetic language across low-end laptops and high-performance multi-monitor workstations.

---

## ⚡ The Three Tiers

| Feature | Tier 1: NEXUS Minimal | Tier 2: NEXUS Core | Tier 3: NEXUS Hyper |
|---|---|---|---|
| **Target Hardware** | 2–4 GB RAM, lightweight laptops | Daily drivers, 8–16 GB RAM | Modern workstations, 16+ GB RAM |
| **Idle Memory Target** | < 180 MB | < 280 MB | < 400 MB |
| **Compositor Engine** | Hyprland (GPU-accel) | Hyprland + native blur | Hyprland + shader plugin stack |
| **Animations** | Fast spring transitions (<6ms) | Full spring physics | Shader transitions + fluid curves |
| **Shell & Panel** | Minimal Quickshell top bar | Modular Bar + Dock + Notifications | Dynamic Shell + iOS Control Center |
| **Widgets** | None | Drag & drop desktop widgets | Lockscreen & desktop interactive widgets |
| **Dynamic Color** | Accent palette | Material You wallpaper extraction | Full Material You + GLSL live wallpapers |

---

## 🏗 Architecture Overview

NEXUS-DE is organized as a high-performance, modular system:

```
┌─────────────────────────────────────────────────────────────┐
│                      NEXUS-DE Shell                         │
│   Quickshell (Qt6/QML) — Bar, Dock, Launcher, OSD, Widgets  │
└──────────────────────────────┬──────────────────────────────┘
                               │ (D-Bus / Unix Socket)
┌──────────────────────────────▼──────────────────────────────┐
│                       nexus-daemon                          │
│   Rust (zbus 4) — State management, Tier Switching,         │
│   Material You palette math, Hyprland IPC bridge            │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                    Hyprland Compositor                      │
│   Wayland compositor with Aquamarine backend                │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 Quick Start (Development)

### Prerequisites

- **Arch Linux / CachyOS / Fedora**
- **Rust toolchain** (`rustc >= 1.80`, `cargo`)
- **Hyprland** (`>= 0.50`)
- **Quickshell** (`quickshell`)
- **GTK4 & Libadwaita** (`gtk4`, `libadwaita-1`)

### Building the Workspace

```bash
# Clone repository
git clone https://github.com/Shyam-vadgama/nexus-de.git
cd nexus-de

# Build all Rust components (daemon, settings, libraries)
cargo build --release

# Run the test suite
cargo test --workspace
```

### Running Tests

```bash
cargo test --workspace
```

---

## 📜 License

NEXUS-DE is licensed under the [GNU General Public License v3.0 or later](LICENSE).
