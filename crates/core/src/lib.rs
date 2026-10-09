

//! Core foundational types and utilities for [niceTrader](https://nicetrader.io).
//!
//! The `nice-core` crate is designed to be lightweight, efficient, and to provide zero-cost abstractions
//! wherever possible. It supplies the essential building blocks used across the niceTrader
//! ecosystem, including:
//!
//! - Time handling and atomic clock functionality.
//! - UUID generation and management.
//! - Mathematical functions and interpolation utilities.
//! - Correctness validation functions.
//! - Serialization traits and codecs.
//! - Cross-platform environment utilities.
//! - Abstractions over common collections.
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
//! - `ffi`: Enables the C foreign function interface (FFI) from
//!   [cbindgen](https://crates.io/crates/cbindgen).
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//! - `simulation`: Enables deterministic simulation testing with
//!   [MadSim](https://crates.io/crates/madsim).

#![warn(rustc::all)]
#![warn(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(
    clippy::inline_always,
    reason = "hot-path predicate guards use #[inline(always)] intentionally for constant-folding"
)]
#![allow(
    clippy::manual_let_else,
    reason = "match can be clearer than let-else for some patterns"
)]
#![allow(
    clippy::assert_is_empty,
    reason = "`assert!(x.is_empty())` is clearer than comparing against an empty value"
)]

pub mod collections;
pub mod consts;
pub mod correctness;
pub mod datetime;
pub mod env;
pub mod hex;
pub mod math;
pub mod nanos;
pub mod params;
pub mod paths;
pub mod serialization;
pub mod shared;
pub mod string;
pub mod time;
pub mod uuid;

#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "python")]
pub mod python;

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_arch = "wasm32"
)))]
compile_error!("Unsupported platform: nice supports only Linux, macOS, Windows, and wasm32");

// Re-exports
#[cfg(feature = "python")]
pub use crate::params::from_pydict;
pub use crate::{
    collections::{AtomicMap, AtomicSet},
    nanos::{DurationNanos, DurationNanosOutOfRangeError, UnixNanos},
    params::Params,
    shared::{SharedCell, WeakCell},
    string::stack_str::{STACKSTR_CAPACITY, StackStr},
    time::AtomicTime,
    uuid::UUID4,
};
