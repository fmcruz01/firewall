# Firewall in Rust
[![Coverage](https://github.com/fmcruz01/firewall/actions/workflows/rust.yml/badge.svg)](https://github.com/fmcruz01/firewall/actions)

## Overview

This firewall is a **self-hosted, modular firewall** written in Rust.  

The project currently focuses on **host-based packet sniffing and filtering** with a roadmap toward **kernel-level gateway firewalling**.

---

## Toolset

- **Language:** Rust
- **Packet capture:** [pcap crate](https://crates.io/crates/pcap)
- **Async & runtime (future phases):** tokio / async-std
- **Testing:** cargo test, Rust unit tests
- **CI/CD:** GitHub Actions

---

## Goals

1. **Learn real networking and security**
2. **Learn Rust**
3. **Create a modular and maintainable firewall**
4. **Bridge user-space and kernel-space packet processing**
5. **Understand defensive and offensive techniques** through controlled testing

---

## Features (Phase 1 → Phase 7)
🟢 - Done
🟡 - In-Progress
🔴 - Not started


| Epic | Goal | Status |
|-------|------|-------|
| 🟩 Epic 1 | Packet Capture (Passive Mode) | 🟡 |
| 🟨 Epic 2 | Packet Parsing (L2 → L4) |🔴|
| 🟧 Epic 3 | Stateless Rule Engine |🔴|
| 🟥 Epic 4 | Packet Blocking (Active Mode) |🔴|
| 🟪 Epic 5 | Stateful Inspection |🔴|
| 🟫 Epic 6 | Logging, Metrics & Alerts |🔴|
| ⬛ Epic 7 | Red Team Testing & Evasion |🔴|

---

## CLI Usage

The firewall CLI is called `firewall` (via `fw-ctl`):

```bash
# Run packet sniffing on interface eth0
firewall sniff --iface eth0 --verbose
