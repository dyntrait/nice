# nice-backtest

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-backtest)](https://docs.rs/nice-backtest/latest/nice_backtest/)
[![crates.io version](https://img.shields.io/crates/v/nice-backtest.svg)](https://crates.io/crates/nice-backtest)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Backtest engine for [niceTrader](https://nicetrader.io).

The `nice-backtest` crate provides an event-driven backtesting framework that allows
quantitative traders to test and validate trading strategies on historical data with high
fidelity market simulation. The system replicates real market conditions including:

- Event-driven backtesting engine with simulated exchanges.
- Market data replay with configurable latency and fill models.
- Order matching engines with realistic execution simulation.
- Multi-venue and multi-asset backtesting capabilities.
- Configuration and state management.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `defi`: Enables DeFi replay APIs and data-engine routing.
- `examples`: Enables example strategies and the EMA crossover backtest example.
- `extension-module`: Builds as a Python extension module.
- `high-precision`: Enables
  [high-precision mode](https://nicetrader.io/docs/nightly/getting_started/installation/#precision-mode)
  to use 128-bit value types.
- `mimalloc`: Uses [mimalloc](https://crates.io/crates/mimalloc) as the global allocator for
  bundled Rust examples.
- `plugin`: Provides a compatibility flag without enabling additional code.
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `streaming`: Enables the `nice-persistence` dependency for streaming configuration.

## Documentation

See [the docs](https://docs.rs/nice-backtest) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
