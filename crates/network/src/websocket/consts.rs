// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2026  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : consts.rs
//  @Author       : dyntrait
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! Constants for WebSocket protocol handling.

/// Standard text ping message.
pub const TEXT_PING: &str = "ping";

/// Standard text pong message.
pub const TEXT_PONG: &str = "pong";

/// Default authentication timeout in seconds.
pub const AUTHENTICATION_TIMEOUT_SECS: u64 = 10;

/// Connection state check interval in milliseconds.
pub(crate) const CONNECTION_STATE_CHECK_INTERVAL_MS: u64 = 10;

/// Send operation check interval in milliseconds.
pub(crate) const SEND_OPERATION_CHECK_INTERVAL_MS: u64 = 1;

/// Graceful shutdown delay in milliseconds.
pub(crate) const GRACEFUL_SHUTDOWN_DELAY_MS: u64 = 100;

/// Graceful shutdown timeout in seconds.
pub(crate) const GRACEFUL_SHUTDOWN_TIMEOUT_SECS: u64 = 5;
