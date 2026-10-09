# nice-testkit

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-testkit)](https://docs.rs/nice-testkit/latest/nice_testkit/)
[![crates.io version](https://img.shields.io/crates/v/nice-testkit.svg)](https://crates.io/crates/nice-testkit)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Test utilities and data management for [niceTrader](https://nicetrader.io).

The `nice-testkit` crate provides testing utilities including test data management,
file handling, and common testing patterns. This crate supports testing workflows
across the entire niceTrader ecosystem with automated data downloads and validation:

- **Test data management**: Automated downloading and caching of test datasets.
- **File utilities**: File integrity verification with SHA-256 checksums.
- **Path resolution**: Platform-agnostic test data path management.
- **Precision handling**: Support for both 64-bit and 128-bit precision test data.
- **Event collection**: Draining and correlating the data events a client emits.
- **Common patterns**: Reusable fixtures and test support.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `datasets` (default): Enables test dataset discovery, download, validation, parsing, and loading.
- `extension-module`: Builds as a Python extension module.
- `high-precision`: Enables
  [high-precision mode](https://nicetrader.io/docs/nightly/getting_started/installation/#precision-mode)
  to use 128-bit value types.
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `testers` (default): Enables test actors, strategies, and in-memory cache backing.

Event collection utilities remain available without enabling a feature.

## Documentation

See [the docs](https://docs.rs/nice-testkit) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
