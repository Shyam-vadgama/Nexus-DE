# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

---

## Reporting a Vulnerability

If you discover a security vulnerability within NEXUS-DE, please report it privately:

1. **Email**: shyamvadgama80@gmail.com
2. **Details**: Include a detailed description of the vulnerability, reproduction steps, affected component (e.g. `nexus-daemon`, lockscreen, or IPC socket), and environment details.
3. Please do **not** open public GitHub issues for security vulnerabilities before they are addressed.

---

## Security Tenets

- **No Privilege Escalation**: NEXUS-DE components run exclusively within the unprivileged user session (`systemd --user`). Any privileged actions (e.g. system power management, package installations) must defer to `polkit` / `logind`.
- **Session Locking Integrity**: The screen lock component relies strictly on the compositor-enforced `ext-session-lock-v1` Wayland protocol, preventing bypass or window leaks.
