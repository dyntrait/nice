# nice-trader

[![crates.io version](https://img.shields.io/crates/v/nice-trader.svg)](https://crates.io/crates/nice-trader)
[![Documentation](https://docs.rs/nice-trader/badge.svg)](https://docs.rs/nice-trader)

Container crate for [niceTrader](https://nicetrader.io).

This crate re-exports the core, model, and common component crates as a small
stable entry point. Use the individual `nice-*` crates for adapter,
backtest, live, and other crate-specific APIs.

The first re-exported modules are:

- `common`: Common machinery from `nice-common`.
- `core`: Core primitives, identifiers, time, and precision support from `nice-core`.
- `model`: Trading domain model and data types from `nice-model`.

Use the other component crates that match your use case:

- `nice-data`: Data engine and market data processing.
- `nice-backtest`: Backtesting machinery.
- `nice-live`: Live trading machinery.
- `nice-trading`: Strategy and actor APIs.
- `nice-execution`: Execution engine and order management.
- `nice-portfolio`: Portfolio accounting.
- `nice-risk`: Risk engine.

Venue adapters publish as separate crates.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade,
Rust-native engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a
single event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `test-support`: Enables model test fixtures, builders, specs, and defaults.

## Documentation

See [the niceTrader documentation](https://nicetrader.io/docs) and the
component crate docs on [docs.rs](https://docs.rs/releases/search?query=nice).

## License

The source code for niceTrader is available on GitHub under the
[GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

Use of this software is subject to the
[Disclaimer](https://nicetrader.io/legal/disclaimer/).
