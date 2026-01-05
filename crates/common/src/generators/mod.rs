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


//! Provides generation of identifiers such as `ClientOrderId` and `PositionId`.

pub mod client_order_id;
pub mod order_list_id;
pub mod position_id;

use chrono::{DateTime, Datelike, Timelike};

fn get_datetime_tag(unix_ms: u64) -> String {
    let now_utc = DateTime::from_timestamp_millis(unix_ms as i64)
        .expect("Milliseconds timestamp should be within valid range");
    format!(
        "{}{:02}{:02}-{:02}{:02}{:02}",
        now_utc.year(),
        now_utc.month(),
        now_utc.day(),
        now_utc.hour(),
        now_utc.minute(),
        now_utc.second(),
    )
}
