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

//! High-performance raw TCP client implementation with TLS capability, automatic reconnection
//! with exponential backoff and state management.

pub mod client;
pub mod config;
pub mod fix;
pub mod types;

pub use client::SocketClient;
pub use config::SocketConfig;
pub use types::{TcpMessageHandler, TcpReader, TcpWriter, WriterCommand};