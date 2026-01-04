// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2026  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : mod.rs
//  @Author       : dyntrait
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
//! WebSocket client implementation with automatic reconnection and subscription tracking.

pub mod auth;
pub mod client;
pub mod config;
pub mod consts;
pub mod subscription;
pub mod types;

// Re-export main types for convenience
pub use auth::AuthTracker;
pub use client::{WebSocketClient, WebSocketClientInner};
pub use config::WebSocketConfig;
pub use consts::{AUTHENTICATION_TIMEOUT_SECS, TEXT_PING, TEXT_PONG};
pub use subscription::{SubscriptionState, split_topic};
pub use types::{MessageHandler, MessageReader, PingHandler, channel_message_handler};
