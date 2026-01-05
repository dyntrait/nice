// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2026  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : mod.rs
//  @Author       : dyntrait
//   @Create       : ${DATE} ${TIME}
//  @Description  :
//
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
//! Status report types for trading operations.
//!
//! This module provides report types for tracking and communicating the status
//! of various trading operations, including order fills, order status, position
//! status, and mass status requests.

pub mod fill;
pub mod mass_status;
pub mod order;
pub mod position;

// Re-exports
pub use fill::FillReport;
pub use mass_status::ExecutionMassStatus;
pub use order::OrderStatusReport;
pub use position::PositionStatusReport;
