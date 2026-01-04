//! # Feature Flags
//!
//! This crate provides feature flags to control source code inclusion during compilation,
//! depending on the intended use case, i.e. whether to provide Python bindings
//! for the [nice_trader](https://pypi.org/project/nice_trader) Python package,
//! or as part of a Rust only build.
//!
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//! - `extension-module`: Builds the crate as a Python extension module.
//! - `turmoil`: Enables deterministic network simulation testing with [turmoil](https://github.com/tokio-rs/turmoil).
//!
//! # Testing
//!
//! The crate includes both standard integration tests and deterministic network simulation tests using turmoil.
//!
//! To run standard tests:
//! ```bash
//! cargo nextest run -p nice-network
//! ```
//!
//! To run turmoil network simulation tests:
//! ```bash
//! cargo nextest run -p nice-network --features turmoil
//! ```
//!
//! The turmoil tests simulate various network conditions (reconnections, partitions, etc.) in a deterministic way,
//! allowing reliable testing of network failure scenarios without flakiness.

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod backoff;
pub mod http;
pub mod mode;
pub mod net;
pub mod retry;
pub mod socket;
pub mod websocket;

mod logging;
mod tls;

#[cfg(feature = "python")]
pub mod python;

pub mod error;
pub mod ratelimiter;

/// Sentinel message to signal reconnection completion to Rust consumers.
pub const RECONNECTED: &str = "__RECONNECTED__";
