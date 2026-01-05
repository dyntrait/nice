// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2025-2026 dyntrait. All rights reserved.
//
//  @File         : signal.rs
//  @Author       : dyntrait Created On 2026/1/5 15:22
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! A user signal type.

use nice::UnixNanos;
use serde::{Deserialize, Serialize};
use ustr::Ustr;

/// Represents a generic signal.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.common")
)]
pub struct Signal {
    pub name: Ustr,
    pub value: String,
    pub ts_event: UnixNanos,
    pub ts_init: UnixNanos,
}

impl Signal {
    /// Creates a new [`Signal`] instance.
    #[must_use]
    pub const fn new(name: Ustr, value: String, ts_event: UnixNanos, ts_init: UnixNanos) -> Self {
        Self {
            name,
            value,
            ts_event,
            ts_init,
        }
    }
}
