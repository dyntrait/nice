# nice-interactive-brokers

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-interactive-brokers)](https://docs.rs/nice-interactive-brokers/latest/nice_interactive_brokers/)
[![crates.io version](https://img.shields.io/crates/v/nice-interactive-brokers.svg)](https://crates.io/crates/nice-interactive-brokers)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

[niceTrader](https://nicetrader.io) adapter for
[Interactive Brokers](https://www.interactivebrokers.com).

The `nice-interactive-brokers` crate wraps the [`ibapi`](https://crates.io/crates/ibapi)
client and connects it to niceTrader's live data, execution, historical data, and instrument
loading infrastructure. Optional PyO3 bindings expose the same implementation through
`nice_trader`.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## What this crate provides

- `data`: `InteractiveBrokersDataClient` for market data subscriptions and live streaming.
- `execution`: `InteractiveBrokersExecutionClient` for order submission, account synchronization,
  and execution updates.
- `historical`: `HistoricalInteractiveBrokersClient` for historical data requests.
- `providers`: `InteractiveBrokersInstrumentProvider` for contract lookup, instrument normalization,
  and symbology conversion.
- `gateway`: `DockerizedIBGateway` for managing a Dockerized IB Gateway when the `gateway` feature
  is enabled.
- `python`: PyO3 bindings exposed through `nice_trader.adapters.interactive_brokers` when the
  `python` feature is enabled.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `examples`: Enables the crate's example binaries.
- `execution` (default): Enables order execution and networking support.
- `extension-module`: Builds the crate as a Python extension module. This is
  the feature used by the `nice_trader` package and includes `python` and
  `gateway`.
- `gateway`: Enables the Dockerized IB Gateway manager via
  [`bollard`](https://crates.io/crates/bollard), including its PyO3 bindings when combined with
  `python`.
- `python`: Enables [PyO3](https://pyo3.rs) bindings for configs, enums, the historical client,
  and the instrument provider.

## Default ports

Use `127.0.0.1` unless you are connecting to a remote host.

| Endpoint              | Trading mode | Default port |
| --------------------- | ------------ | -----------: |
| IB Gateway            | Paper        |       `4002` |
| IB Gateway            | Live         |       `4001` |
| TWS                   | Paper        |       `7497` |
| TWS                   | Live         |       `7496` |
| Dockerized IB Gateway | Paper        |       `4002` |
| Dockerized IB Gateway | Live         |       `4001` |

This crate defaults to `4002`, which matches paper-trading IB Gateway and the
default Dockerized IB Gateway paper setup. If you are connecting to TWS or to a
live Gateway session, set the port explicitly in your config.

## Market data timestamps

Configure TWS or IB Gateway to return market data timestamps in UTC before connecting
niceTrader. The adapter does not convert these timestamps automatically at runtime.

## Documentation

- [Crate docs](https://docs.rs/nice-interactive-brokers): generated Rust API reference.
- [Interactive Brokers integration guide](https://nicetrader.io/docs/nightly/integrations/interactive_brokers/):
  setup, configuration, symbology, and usage.
- [Rust node examples](examples): live data and execution testers.
- [Python live-node examples](https://github.com/nautechsystems/nice_trader/tree/develop/examples/live/interactive_brokers):
  Python configuration examples.

## License

The source code for niceTrader is available on GitHub under the
[GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
