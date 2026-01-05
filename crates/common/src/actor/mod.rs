// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2025-2026 dyntrait. All rights reserved.
//
//  @File         : mod.rs.rs
//  @Author       : dyntrait Created On 2026/1/5 15:13
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------


//! Actor system for event-driven message processing.
//!
//! This module provides the actor framework used throughout NautilusTrader for handling
//! data processing, event management, and asynchronous message handling. Actors are
//! lightweight components that process messages in isolation.

#![allow(unsafe_code)]

use std::{any::Any, fmt::Debug};

use ustr::Ustr;

pub mod data_actor;
#[cfg(feature = "indicators")]
pub(crate) mod indicators;
pub mod registry;

#[cfg(test)]
mod tests;

// Re-exports
pub use data_actor::{DataActor, DataActorConfig, DataActorCore};

pub use crate::component::Component;

pub trait Actor: Any + Debug {
    /// The unique identifier for the actor.
    fn id(&self) -> Ustr;
    /// Handles the `msg`.
    fn handle(&mut self, msg: &dyn Any);
    /// Returns a reference to `self` as `Any`, for downcasting support.
    fn as_any(&self) -> &dyn Any;
    /// Returns a mutable reference to `self` as `Any`, for downcasting support.
    ///
    /// Default implementation simply coerces `&mut Self` to `&mut dyn Any`.
    ///
    /// # Note
    ///
    /// This method is not object-safe and thus only available on sized `Self`.
    fn as_any_mut(&mut self) -> &mut dyn Any
    where
        Self: Sized,
    {
        self
    }
}
