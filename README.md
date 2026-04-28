<div align="center">
  <img width="200" height="200" alt="rscan_logo" src="https://github.com/user-attachments/assets/bf70684b-3241-47cc-9369-c8705d90bd36" />
  <h1> rscan </h1>
  <img src="https://github.com/randomctl/rscan/actions/workflows/rust.yml/badge.svg?branch=main" />
  <img src="https://img.shields.io/github/last-commit/randomctl/rscan" />
</div>

<br />

`rscan` is a network security scanning tool written in Rust.

The goal is to build an efficient network scanner for local LAN environments with:
- host and service discovery
- packet inspection
- built-in threat detection
- terminal user interface (TUI)

This project is currently being reorganized into a crate-based architecture.

## Usage

```bash
sudo cargo run --bin rscan -- sniff
```

## Crates

- `core`: orchestration/runtime integration, shared models/events, config and security checks
- `capture`: packet capture and frame decode/parsing adapters
- `probe`: active discovery and probing workflow (host/port scan pipeline)
- `detect`: detection engine and rule evaluation
- `tui`: terminal UI surface and input handling

