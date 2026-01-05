// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : build.rs
//  @Author       : dyntrait
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#![allow(clippy::needless_return)]

//! Build script for the `nice-core` crate.
//!
//! This script is executed by Cargo during compilation and is responsible for the ancillary
//! tasks that the core library requires in order to compile correctly across the various
//! combinations of features and target environments supported by NiceTrader.
//!
//! Specifically it performs the following duties:
//!
//! 1. Propagates version information from the top-level `pyproject.toml` (when available) so it
//!    can be embedded in the compiled binary via the `NICE_VERSION` and
//!    `NICE_USER_AGENT` environment variables.
//! 2. Generates C and Cython headers when the `ffi` feature flag is enabled.  The bindings are
//!    produced with [`cbindgen`](https://github.com/mozilla/cbindgen) and written into the
//!    Python package tree so that users building Python wheels do not need to have a Rust
//!    toolchain installed.
//! 3. Emits the appropriate `cargo:rerun-if-*` directives so that Cargo reruns this build script
//!    whenever any of the relevant environment variables or configuration files change.
//!
//! The script exits early when it detects the `DOCS_RS` environment variable, as header
//! generation is unnecessary (and sometimes not permitted) in the docs.rs build sandbox.



#[allow(clippy::expect_used)]
fn main() {
    // Ensure the build script runs on changes
    println!("cargo:rerun-if-env-changed=HIGH_PRECISION");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_HIGH_PRECISION");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=../Cargo.toml");

    let nice_version = "1.222.0"; // Hardcode to avoid including pyproject.toml in package

    // Set compile-time environment variables
    println!("cargo:rustc-env=NICE_VERSION={nice_version}");
    println!("cargo:rustc-env=NICE_USER_AGENT=NiceTrader/{nice_version}");

    // Skip file generation if we're in the docs.rs environment
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Running in docs.rs environment, skipping file generation");
        return;
    }
}

