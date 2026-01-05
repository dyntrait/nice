// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2025-2026 dyntrait. All rights reserved.
//
//  @File         : custom.rs
//  @Author       : dyntrait Created On 2026/1/5 15:17
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! A user custom data type.

use bytes::Bytes;
use nice_core::UnixNanos;
use nice_model::data::DataType;
use serde::{Deserialize, Serialize};

/// Represents a custom data.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nice_trader.core.nice_pyo3.common")
)]
pub struct CustomData {
    pub data_type: DataType,
    pub value: Bytes,
    pub ts_event: UnixNanos,
    pub ts_init: UnixNanos,
}

impl CustomData {
    /// Creates a new [`CustomData`] instance.
    pub const fn new(
        data_type: DataType,
        value: Bytes,
        ts_event: UnixNanos,
        ts_init: UnixNanos,
    ) -> Self {
        Self {
            data_type,
            value,
            ts_event,
            ts_init,
        }
    }
}
