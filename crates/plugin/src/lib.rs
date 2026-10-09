

//! Plug-in artifact identity and boundary primitives for
//! [niceTrader](https://nicetrader.io).
//!
//! This crate provides the public contract that lets an independently compiled
//! Rust cdylib identify itself to a nice host. It defines versioned build
//! metadata, allocator-safe boundary values, opaque host tokens, and the
//! `nice_plugin!` macro for exporting the standard entry symbol and
//! manifest.
//!
//! # Feature Flags
//!
//! This crate provides feature flags to control source code inclusion during compilation:
//!
//! - `component-binding`: Enables the experimental executable component contract.
//! - `host`: Optional plug-in manifest compatibility flag.

#![warn(clippy::pedantic)]
#![allow(
    clippy::assert_is_empty,
    reason = "`assert!(x.is_empty())` is clearer than comparing against an empty value"
)]

/// ABI version of the public plug-in metadata contract.
///
/// The host refuses to load a plug-in whose
/// [`PluginManifest::abi_version`](crate::manifest::PluginManifest::abi_version)
/// does not match this value.
pub const NICE_PLUGIN_ABI_VERSION: u32 = 1;

/// Schema version for [`manifest::PluginBuildId`].
pub const PLUGIN_BUILD_ID_VERSION: u32 = 1;

/// Name of the single `extern "C"` entry symbol every plug-in cdylib exports.
pub const NICE_PLUGIN_INIT_SYMBOL: &[u8] = b"nice_plugin_init";

pub mod boundary;
#[cfg(feature = "component-binding")]
#[doc(hidden)]
pub mod component;
pub mod host;
pub mod manifest;
pub mod panic;

mod macros;

pub use boundary::{BorrowedStr, OwnedBytes, PluginError, PluginErrorCode, PluginResult, Slice};
pub use host::{HostContext, HostVTable};
pub use manifest::{PluginBuildId, PluginInitFn, PluginManifest};

/// Re-exports that plug-in crates typically want in scope.
pub mod prelude {
    pub use crate::{
        BorrowedStr, HostContext, HostVTable, NICE_PLUGIN_ABI_VERSION, PluginBuildId,
        PluginError, PluginErrorCode, PluginManifest, PluginResult, Slice,
    };
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn init_symbol_matches_exported_entrypoint() {
        assert_eq!(NICE_PLUGIN_INIT_SYMBOL, b"nice_plugin_init");
    }
}
