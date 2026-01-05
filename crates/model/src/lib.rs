// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : lib.rs
//  @Author       : dyntrait
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------


//!
//! # Feature Flags
//!
//! This crate provides feature flags to control source code inclusion during compilation,

//! or as part of a Rust only build.
//!
//! - `stubs`: Enables type stubs for use in testing scenarios.
//! - `high-precision`: Enables [high-precision mode] to use 128-bit value types.
//! - `defi`: Enables the DeFi (Decentralized Finance) domain model.


#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod accounts;
pub mod currencies;
pub mod data;
pub mod enums;
pub mod events;
pub mod identifiers;
pub mod instruments;
pub mod macros;
pub mod orderbook;
pub mod orders;
pub mod position;
pub mod reports;
pub mod types;
pub mod venues;
#[cfg(any(test, feature = "stubs"))]
pub mod stubs;

#[cfg(feature = "defi")]
pub mod defi;
