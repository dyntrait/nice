// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2025-2026 dyntrait. All rights reserved.
//
//  @File         : request.rs
//  @Author       : dyntrait Created On 2026/1/5 16:06
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

use indexmap::IndexMap;
use nice_core::{UUID4, UnixNanos};
use nice_model::identifiers::{ClientId, InstrumentId};

/// Represents a request for a pool snapshot from a specific AMM pool.
#[derive(Clone, Debug)]
pub struct RequestPoolSnapshot {
    pub instrument_id: InstrumentId,
    pub client_id: Option<ClientId>,
    pub request_id: UUID4,
    pub ts_init: UnixNanos,
    pub params: Option<IndexMap<String, String>>,
}

impl RequestPoolSnapshot {
    /// Creates a new [`RequestPoolSnapshot`] instance.
    #[must_use]
    pub const fn new(
        instrument_id: InstrumentId,
        client_id: Option<ClientId>,
        request_id: UUID4,
        ts_init: UnixNanos,
        params: Option<IndexMap<String, String>>,
    ) -> Self {
        Self {
            instrument_id,
            client_id,
            request_id,
            ts_init,
            params,
        }
    }
}
