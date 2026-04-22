# rscan

Network security scanning tool written in rust. The goal is to make an efficient and effective tool to scan a network against intruders, pakcet injections and so on.
Currently doing some network research if feasible and how to achieve this.
Requires scanner (host/service discovery), packet inspection, detection engine.

## Crates

- `core`: packet parsing primitives (Ethernet, IPv4/IPv6, TCP/UDP)
- `runtime`: capture loop and runtime wiring
- `cli`: command-line entry point

## Usage

```bash
sudo cargo run --bin rscan -- sniff
```
