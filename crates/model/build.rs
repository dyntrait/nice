// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : build.rs
//  @Author       : mark.m
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! Build script for the `nautilus-model` crate.
//!
//! In addition to the common tasks performed by the other build scripts (header generation,
//! rerun-tracking, docs.rs early-exit) this script also toggles *high-precision* mode for the
//! generated bindings based on either:
//!
//! 1. The `HIGH_PRECISION` environment variable, **or**
//! 2. The compile-time `high-precision` cargo feature.
//!
//! When enabled the flag is forwarded to the Cython bindings via a `DEF HIGH_PRECISION` macro so
//! that the Python layer compiles in a compatible configuration.



#[allow(
    clippy::expect_used,
    reason = "Build script may panic on misconfiguration"
)]
#[allow(
    unused_assignments,
    reason = "Conditional compilation creates unused assignments"
)]
#[allow(unused_mut)]
fn main() {
    // Skip file generation if we're in the docs.rs environment
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Running in docs.rs environment, skipping file generation");
        return;
    }

    // Ensure the build script runs on changes
    println!("cargo:rerun-if-env-changed=HIGH_PRECISION");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_HIGH_PRECISION");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../Cargo.toml");
}
