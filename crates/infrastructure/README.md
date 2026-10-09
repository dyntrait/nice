# nice-infrastructure

[![build](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nice_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nice-infrastructure)](https://docs.rs/nice-infrastructure/latest/nice_infrastructure/)
[![crates.io version](https://img.shields.io/crates/v/nice-infrastructure.svg)](https://crates.io/crates/nice-infrastructure)
![license](https://img.shields.io/github/license/nautechsystems/nice_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/niceTrader)

Database and messaging infrastructure for [niceTrader](https://nicetrader.io).

The `nice-infrastructure` crate provides backend database implementations and message bus adapters
that enable niceTrader to scale from development to production deployments. This includes
enterprise-grade data persistence and messaging capabilities:

- **Redis integration**: Cache database and message bus implementations using Redis.
- **PostgreSQL integration**: SQL-based cache database with full data models.
- **Connection management**: Connection handling with retry logic and health monitoring.
- **Serialization options**: Support for JSON and MessagePack encoding formats.
- **Python bindings**: PyO3 integration for Python interoperability.

The crate supports multiple database backends through feature flags, allowing users to choose
the appropriate infrastructure components for their specific deployment requirements and scale.

## niceTrader

[niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
engine for multi-asset, multi-venue trading systems.

The system spans research, deterministic simulation, and live execution within a single
event-driven architecture, providing research-to-live semantic parity.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation:

- `extension-module`: Builds as a Python extension module.
- `postgres`: Enables the PostgreSQL SQLx models and cache database backend.
- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `redis` (default): Enables the Redis cache database and message bus backing implementations.

## Documentation

See [the docs](https://docs.rs/nice-infrastructure) for more detailed usage.

## License

The source code for niceTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).

---

niceTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nicetrader.io>.

Use of this software is subject to the [Disclaimer](https://nicetrader.io/legal/disclaimer/).

<img src="https://github.com/nautechsystems/nice_trader/raw/develop/assets/nice-logo-white.png" alt="logo" width="300" height="auto"/>

© 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
