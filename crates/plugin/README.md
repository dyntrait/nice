# nice-plugin

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-plugin)](https://docs.rs/nice-plugin/latest/nice_plugin/)
[![crates.io version](https://img.shields.io/crates/v/nice-plugin.svg)](https://crates.io/crates/nice-plugin)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Plug-in artifact identity and boundary primitives for
[niceTrader](https://nicetrader.io).

The `nice-plugin` crate provides the public contract that lets an independently compiled Rust
cdylib carry a versioned identity. It defines versioned build metadata, allocator-safe boundary
values, opaque boundary tokens, and the `nice_plugin!` macro for exporting the standard entry
symbol and manifest.

This crate gives plug-in artifacts a consistent identity and a compact contract.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `component-binding`: Enables the experimental executable component contract.
- `host`: Optional plug-in manifest compatibility flag.

## Documentation

See [the docs](https://docs.rs/nice-plugin) for the API reference.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
