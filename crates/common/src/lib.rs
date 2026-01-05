// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//  @File         : lib.rs
//  @Author       : dyntrait
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------


#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod actor;
pub mod cache;
pub mod clock;
pub mod component;
pub mod custom;
pub mod enums;
pub mod factories;
pub mod generators;
pub mod greeks;
pub mod logging;
pub mod messages;
pub mod msgbus;
pub mod runner;
pub mod signal;
pub mod testing;
pub mod throttler;
pub mod timer;
pub mod xrate;

#[cfg(feature = "live")]
pub mod live;

#[cfg(feature = "defi")]
pub mod defi;



#[cfg(feature = "capnp")]
pub mod serialization;
