// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2025-2026 dyntrait. All rights reserved.
//
//  @File         : mod.rs.rs
//  @Author       : dyntrait Created On 2026/1/5 15:14
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
//! Live (async/tokio) components for real-time trading.
//!
//! This module contains components that require the tokio async runtime and are
//! used for live trading scenarios. These are gated behind the `live` feature flag.

pub mod clock;
pub mod listener;
pub mod runner;
pub mod runtime;
pub mod timer;

pub use clock::{LiveClock, TimeEventStream};
pub use listener::MessageBusListener;
pub use runner::{
    get_data_event_sender, get_exec_event_sender, set_data_event_sender, set_exec_event_sender,
};
pub use runtime::{get_runtime, shutdown_runtime};
pub use timer::LiveTimer;
