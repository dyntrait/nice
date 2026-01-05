// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2025-2026 dyntrait. All rights reserved.
//
//  @File         : mod.rs
//  @Author       : dyntrait Created On 2026/1/5 15:10
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! Message types for system communication.
//!
//! This module provides message types used for communication between different
//! parts of the niceTrader system, including data requests, execution commands,
//! and system control messages.

use nice_model::{
    data::{Data, FundingRateUpdate},
    events::{AccountState, OrderEventAny},
    instruments::InstrumentAny,
};
use strum::Display;

pub mod data;
pub mod execution;
pub mod system;

#[cfg(feature = "defi")]
pub mod defi;

// Re-exports
pub use data::{DataResponse, SubscribeCommand, UnsubscribeCommand};
pub use execution::ExecutionReport;

// TODO: Refine this to reduce disparity between enum sizes
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Display)]
pub enum DataEvent {
    Response(DataResponse),
    Data(Data),
    Instrument(InstrumentAny), // TODO: Eventually this can be `Data` once Cython is gone
    FundingRate(FundingRateUpdate),
    // nice-import-ok: conditional compilation import
    #[cfg(feature = "defi")]
    DeFi(nice_model::defi::data::DefiData),
}

/// Execution event variants for order events and reports.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Display)]
pub enum ExecutionEvent {
    Order(OrderEventAny),
    Report(ExecutionReport),
    Account(AccountState),
}
