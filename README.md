# FDCAN peripheral driver

[![crates.io](https://img.shields.io/crates/v/fdcan.svg)](https://crates.io/crates/fdcan)
[![docs.rs](https://docs.rs/fdcan/badge.svg)](https://docs.rs/fdcan/)
![CI](https://github.com/stm32-rs/fdcan/actions/workflows/ci.yml/badge.svg)

This crate implements a driver for the FDCAN peripheral found in high-end STM32
microcontrollers (G0, G4, H7, L5 series). The other CAN peripheral found on
STM32 microcontrollers is [bxCAN](https://github.com/stm32-rs/bxcan).

## Usage

Add an entry to your `Cargo.toml`:

```toml
[dependencies]
fdcan = "0.1.2"
```

# Minimum supported Rust version

The Minimum Supported Rust Version (MSRV) at the moment is **1.52.0**. Older
versions **may** compile, especially when some features are not used
in your application.

# Changelog

See [CHANGELOG.md](CHANGELOG.md).
