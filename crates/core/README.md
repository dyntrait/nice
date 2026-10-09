# nice-core

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-core)](https://docs.rs/nice-core/latest/nice_core/)
[![crates.io version](https://img.shields.io/crates/v/nice-core.svg)](https://crates.io/crates/nice-core)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Core foundational types and utilities for [niceTrader](https://nicetrader.io).

The `nice-core` crate is designed to be lightweight, efficient, and to provide zero-cost abstractions
wherever possible. It supplies the essential building blocks used across the niceTrader
ecosystem, including:

- Time handling and atomic clock functionality.
- UUID generation and management.
- Mathematical functions and interpolation utilities.
- Correctness validation functions.
- Serialization traits and codecs.
- Cross-platform environment utilities.
- Abstractions over common collections.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `extension-module`: Builds as a Python extension module.
- `ffi`: Enables the C foreign function interface (FFI) from [cbindgen](https://crates.io/crates/cbindgen).
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `simulation`: Enables deterministic simulation testing with
  [MadSim](https://crates.io/crates/madsim).

## Documentation

See [the docs](https://docs.rs/nice-core) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
