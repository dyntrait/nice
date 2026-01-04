// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2026  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : error.rs
//  @Author       : dyntrait
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
//! Network error types.

use thiserror::Error;

/// Error type for send operations in network clients.
#[derive(Error, Debug)]
pub enum SendError {
    /// The client has been closed or is disconnecting.
    #[error("send failed: client closed or disconnecting")]
    Closed,
    /// Timed out waiting for the client to become active.
    #[error("send failed: timeout waiting for active state")]
    Timeout,
    /// Failed to send because the writer channel is closed.
    #[error("send failed: broken pipe ({0})")]
    BrokenPipe(String),
}
