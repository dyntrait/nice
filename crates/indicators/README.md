# nice-indicators

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-indicators)](https://docs.rs/nice-indicators/latest/nice_indicators/)
[![crates.io version](https://img.shields.io/crates/v/nice-indicators.svg)](https://crates.io/crates/nice-indicators)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Technical analysis indicators for [niceTrader](https://nicetrader.io).

The `nice-indicators` crate provides a collection of technical analysis indicators
for quantitative trading and market research. This includes a wide variety of indicators
organized by category, with a unified trait-based architecture for consistent usage:

- **Moving averages**: SMA, EMA, DEMA, HMA, WMA, VWAP, adaptive averages, linear regression,
  and z-score.
- **Momentum indicators**: RSI, MACD, Aroon, Bollinger Bands, CCI, Stochastics, and rate of change.
- **Volatility indicators**: ATR, Donchian Channels, Keltner Channels, and volatility ratios.
- **Ratio analysis**: Efficiency ratios and spread analysis for relative performance.
- **Order book indicators**: Book imbalance ratio for analyzing market microstructure.
- **Common indicator trait**: Unified interface supporting bars, quotes, trades, and order book data.

All indicators are designed for high-performance real-time processing with bounded memory
usage and efficient circular buffer implementations. The crate supports both Rust-native
usage and Python integration for strategy development and backtesting.

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

See [the docs](https://docs.rs/nice-indicators) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
