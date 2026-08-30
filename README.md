<div align="center">
  <img width="200" height="200" alt="rscan_logo" src="https://github.com/user-attachments/assets/bf70684b-3241-47cc-9369-c8705d90bd36" />
  <h1> rscan </h1>
  <img src="https://github.com/rndomd/rscan/actions/workflows/rust.yml/badge.svg?branch=main" />
  <img src="https://img.shields.io/github/last-commit/rndomd/rscan" />
</div>

<br />

`rscan` is a LAN scanning tool written in Rust.

The goal is to build an efficient network scanner for LAN environments that supports dicovering of all devices on the local network
and fingerprinting each device to gather metadata on it.

By running `rscan discover` the default network interface will be used to search all the possible IP addresses in the network. It will only scan for currently
actively connected devices.

On the other hand `rscan fingerprint <IP>` will scan against the given IP, similar to nmap, to fingerprint the device. The concrete output of this command is yet to be decided.
