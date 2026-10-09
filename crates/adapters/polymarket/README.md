# nice-polymarket

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-polymarket)](https://docs.rs/nice-polymarket/latest/nice_polymarket/)
[![crates.io version](https://img.shields.io/crates/v/nice-polymarket.svg)](https://crates.io/crates/nice-polymarket)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

[niceTrader](https://nicetrader.io) adapter for the [Polymarket](https://polymarket.com) prediction market.

The `nice-polymarket` crate provides client implementations (HTTP & WebSocket), data
models and parsing for the **Polymarket CLOB API** for trading binary option contracts.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `examples`: Enables the crate's example binaries.
- `extension-module`: Builds as a Python extension module.
- `high-precision` (default): Enables
  [high-precision mode](https://nicetrader.io/docs/nightly/getting_started/installation/#precision-mode)
  to use 128-bit value types.
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).

## API endpoints

The adapter communicates with four Polymarket API surfaces:

| API            | Base URL                                        | Auth                   | Purpose                                     |
| -------------- | ----------------------------------------------- | ---------------------- | ------------------------------------------- |
| CLOB REST      | `https://clob.polymarket.com`                   | L2 HMAC                | Orders, trades, balances.                   |
| CLOB WebSocket | `wss://ws-subscriptions-clob.polymarket.com/ws` | L2 HMAC (user channel) | Streaming orderbook, trades, order updates. |
| Gamma          | `https://gamma-api.polymarket.com`              | None                   | Market and event discovery, tags, search.   |
| Data           | `https://data-api.polymarket.com`               | None                   | Trade history and user positions.           |

## Authentication

Polymarket uses two-tier authentication:

- **L1 (EIP-712)**: Wallet-level signing for API credential creation and order signing
  via the CTF Exchange contract. Uses `alloy` signer crates.
- **L2 (HMAC-SHA256)**: API key + secret + passphrase for authenticated REST and
  WebSocket requests. Signatures expire after 30 seconds.

## Documentation

See [the docs](https://docs.rs/nice-polymarket) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
