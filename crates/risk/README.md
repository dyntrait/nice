# nice-risk

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-risk)](https://docs.rs/nice-risk/latest/nice_risk/)
[![crates.io version](https://img.shields.io/crates/v/nice-risk.svg)](https://crates.io/crates/nice-risk)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Risk engine for [niceTrader](https://nicetrader.io).

The `nice-risk` crate provides risk management capabilities including pre-trade
order validation, position sizing calculations, and trading controls. This system ensures
trading operations remain within defined risk parameters and regulatory constraints:

- **Risk engine**: Central risk management orchestration with configurable trading states.
- **Order validation**: Pre-trade checks for price, quantity, notional limits, and market conditions.
- **Position sizing**: Fixed-risk position sizing calculations with commission and exchange rate support.
- **Trading controls**: Rate limiting, balance validation, and exposure management.
- **Account protection**: Multi-currency balance checks and margin requirement validation.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `extension-module`: Builds as a Python extension module.
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).

## Documentation

See [the docs](https://docs.rs/nice-risk) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
