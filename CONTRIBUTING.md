# Contributing to NEXUS-DE

Thank you for your interest in contributing to NEXUS-DE! We welcome contributions from developers, designers, and Linux enthusiasts.

---

## 🛠 Code of Conduct & Standards

1. **Safety First**: All system-level code should prefer Rust. Where C++ is required (Hyprland plugin), enforce strict boundary checks and RAII.
2. **Wayland Native**: We do not write X11-dependent fallbacks. NEXUS-DE targets pure Wayland via standard protocols (`wlr-layer-shell`, `ext-session-lock-v1`).
3. **No Breaking Crashes**: Shell components must fail gracefully. A crashing widget or plugin must never kill the active compositor session.

---

## 📌 Commit Message Format

We strictly adhere to [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>
```

### Types:
- `feat`: A new feature or capability
- `fix`: A bug fix
- `test`: Adding or refactoring tests
- `docs`: Documentation updates
- `refactor`: Code changes that neither fix bugs nor add features
- `chore`: Build tools, CI, dependency updates

### Examples:
- `feat(daemon): implement zbus D-Bus server for SetTier`
- `fix(ipc): handle socket disconnect on Hyprland reload`
- `test(core): add OkHSL color extraction tests`
- `chore(ci): configure cargo clippy and test workflows`

---

## 🧪 Testing Before Submitting PRs

Run the following checks locally:

```bash
# Format check
cargo fmt --all -- --check

# Linter
cargo clippy --workspace --all-targets -- -D warnings

# Tests
cargo test --workspace
```
