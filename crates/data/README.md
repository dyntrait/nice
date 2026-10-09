# nice-data

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-data)](https://docs.rs/nice-data/latest/nice_data/)
[![crates.io version](https://img.shields.io/crates/v/nice-data.svg)](https://crates.io/crates/nice-data)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Data engine and market data processing for [niceTrader](https://nicetrader.io).

The `nice-data` crate provides a framework for handling market data ingestion,
processing, and aggregation within the niceTrader ecosystem. This includes real-time
data streaming, historical data management, and various aggregation methodologies:

- High-performance data engine for orchestrating data operations.
- Data client infrastructure for connecting to market data providers.
- Bar aggregation machinery supporting tick, volume, value, and time-based aggregation.
- Order book management and delta processing capabilities.
- Subscription management and data request handling.
- Configurable data routing and processing pipelines.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `defi`: Enables DeFi (Decentralized Finance) support.
- `extension-module`: Enables Python extension-module support.
- `high-precision`: Enables
  [high-precision mode](https://nicetrader.io/docs/nightly/getting_started/installation/#precision-mode)
  to use 128-bit value types.
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `streaming`: Enables the `nice-persistence` dependency for catalog-based data streaming.

## Documentation

See [the docs](https://docs.rs/nice-data) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
