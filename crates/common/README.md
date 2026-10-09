# nice-common

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-common)](https://docs.rs/nice-common/latest/nice_common/)
[![crates.io version](https://img.shields.io/crates/v/nice-common.svg)](https://crates.io/crates/nice-common)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Common componentry for [niceTrader](https://nicetrader.io).

The `nice-common` crate provides shared components and utilities that form the system foundation for
niceTrader applications. This includes the actor system, message bus, caching layer, and other
essential services.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `build-info-event-store`: Includes the event-store backend version in build information logs.
- `capnp`: Enables [Cap'n Proto](https://capnproto.org) serialization support.
- `defi`: Enables DeFi (Decentralized Finance) support.
- `extension-module`: Builds as a Python extension module.
- `high-precision`: Enables
  [high-precision mode](https://nicetrader.io/docs/nightly/getting_started/installation/#precision-mode)
  to use 128-bit value types.
- `indicators`: Includes the `nice-indicators` crate and indicator utilities.
- `live`: Enables the Tokio async runtime for live trading.
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `sbe`: Enables Simple Binary Encoding (SBE) serialization support.
- `simulation`: Enables deterministic simulation testing with
  [MadSim](https://crates.io/crates/madsim).
- `tracing-bridge`: Enables the `tracing` subscriber bridge for log integration.

## Documentation

See [the docs](https://docs.rs/nice-common) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
