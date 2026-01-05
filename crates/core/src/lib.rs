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
//! Core utilities for the Nice project.
#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
pub mod collections;
pub mod datetime;
pub mod nanos;
pub mod uuid;
pub mod time;
pub mod consts;
pub mod correctness;
pub mod drop;
pub mod math;
pub mod env;
pub mod message;
pub mod parsing;
pub mod paths;
pub mod serialization;
pub mod shared;
pub mod string;
pub mod formatting;
pub mod stack_str;

// Re-exports
pub use crate::{
    nanos::UnixNanos,
    time::AtomicTime,
    drop::CleanDrop,
    uuid::UUID4,
    shared::{SharedCell,WeakCell},
    message::Params,
    stack_str::{STACKSTR_CAPACITY, StackStr},

};

/// Message for when a mutex guard cannot be acquired due to poisoning.
///
/// Mutex guards should use `expect` rather than handle poison errors.
/// A poisoned mutex indicates a thread panicked while holding the lock,
/// meaning protected data may be in an inconsistent state. Propagating
/// the panic is the idiomatic and safe approach, as continuing with
/// potentially corrupted data would violate safety invariants.
pub const MUTEX_POISONED: &str = "Mutex poisoned";