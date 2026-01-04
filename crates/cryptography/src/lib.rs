//! Cryptographic utilities and security functions for [NautilusTrader](http://nautilustrader.io).
//!
//! The `nice-cryptography` crate provides essential cryptographic primitives and security utilities
//! required for secure communication with trading venues and data providers. This includes
//! digital signing, TLS configuration, and cryptographic provider management:
//!
//! - HMAC-based message authentication and signing.
//! - Digital signatures using RSA and Ed25519 algorithms.
//! - TLS client configuration with platform certificate verification.
//! - Cryptographic provider management and initialization.
//! -

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod providers;
pub mod signing;
pub mod tls;

