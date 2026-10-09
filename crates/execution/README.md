# nice-execution

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-execution)](https://docs.rs/nice-execution/latest/nice_execution/)
[![crates.io version](https://img.shields.io/crates/v/nice-execution.svg)](https://crates.io/crates/nice-execution)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Order execution engine for [niceTrader](https://nicetrader.io).

The `nice-execution` crate provides an order execution system that handles the complete
order lifecycle from submission to fill processing. This includes sophisticated order matching,
execution venue integration, and advanced order type emulation:

- **Execution engine**: Central orchestration of order routing and position management.
- **Order matching engine**: High-fidelity market simulation for backtesting and paper trading.
- **Order emulator**: Advanced order types not natively supported by venues (trailing stops, contingent orders).
- **Execution clients**: Abstract interfaces for connecting to trading venues and brokers.
- **Order manager**: Local order lifecycle management and state tracking.
- **Matching core**: Low-level order book and price-time priority matching algorithms.
- **Fee and fill models**: Configurable execution cost simulation and realistic fill behavior.

The crate supports both live trading environments (with real execution clients) and simulated
environments (with matching engines), making it suitable for production trading, strategy
development, and backtesting.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `extension-module`: Builds as a Python extension module.
- `high-precision`: Enables
  [high-precision mode](https://nicetrader.io/docs/nightly/getting_started/installation/#precision-mode)
  to use 128-bit value types.
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `simulation`: Enables deterministic simulation testing with
  [MadSim](https://crates.io/crates/madsim).

## Documentation

See [the docs](https://docs.rs/nice-execution) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
