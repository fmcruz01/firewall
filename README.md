<div align="center">
  <img width="200" height="200" alt="rscan_logo" src="https://github.com/user-attachments/assets/bf70684b-3241-47cc-9369-c8705d90bd36" />
  <h1> rscan </h1>
  <img src="https://github.com/randomctl/rscan/actions/workflows/rust.yml/badge.svg?branch=main" />
  <img src="https://img.shields.io/github/last-commit/randomctl/rscan" />
</div>

<br />

`rscan` is a LAN scanning tool written in Rust.

The goal is to build an efficient network scanner for LAN environments that supports dicovering of all devices on the local network
and fingerprinting each device to gather metadata on it.

## Usage

```
RSCAN v1.0 -- LAN Scanning Tool

Usage: rscan [OPTIONS] <COMMAND>

Commands:
  discover     Scans LAN to discover devices
  fingerprint  Lists all information about device with provided id
  help         Print this message or the help of the given subcommand(s)

Options:
  -o, --output <OUTPUT>  Path to the output file. If file does not exist, one will be created
  -h, --help             Print help
  -V, --version          Print version
```

### Discover command

```
Scans LAN to discover devices

Usage: rscan discover [OPTIONS]

Options:
  -i, --interface <INTERFACE>  [default: en0]
  -h, --help                   Print help
  -V, --version                Print version
```

### Fingerprint command

```
Lists all information about device with provided id

Usage: rscan fingerprint <ID>

Arguments:
  <ID>  id of the device to fingerprint

Options:
  -h, --help     Print help
  -V, --version  Print version
```

