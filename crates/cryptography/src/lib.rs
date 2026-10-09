

//! Cryptographic utilities and security functions for [niceTrader](https://nicetrader.io).
//!
//! The `nice-cryptography` crate provides essential cryptographic primitives and security utilities
//! required for secure communication with trading venues and data providers. This includes
//! digital signing, TLS configuration, and cryptographic provider management:
//!
//! - HMAC-based message authentication and signing.
//! - Digital signatures using RSA and Ed25519 algorithms.
//! - TLS client configuration with platform certificate verification.
//! - Cryptographic provider management and initialization.
//! - Secure encoding and decoding utilities.
//!
//! # niceTrader
//!
//! [niceTrader](https://nicetrader.io) is an open-source, production-grade, Rust-native
//! engine for multi-asset, multi-venue trading systems.
//!
//! The system spans research, deterministic simulation, and live execution within a single
//! event-driven architecture, providing research-to-live semantic parity.
//!
//! # Feature Flags
//!
//! This crate provides feature flags to control source code inclusion during compilation,
//! depending on the intended use case, i.e. whether to provide Python bindings
//! for the [nice_trader](https://pypi.org/project/nice_trader) Python package,
//! or as part of a Rust only build.
//!
//! - `extension-module`: Builds as a Python extension module.
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).

#![warn(rustc::all)]
#![warn(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(
    clippy::assert_is_empty,
    reason = "`assert!(x.is_empty())` is clearer than comparing against an empty value"
)]

pub mod providers;
pub mod signing;
pub mod tls;


