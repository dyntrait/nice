// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2026  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : types.rs
//  @Author       : dyntrait
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! Socket types and type aliases.

use std::sync::Arc;

use bytes::Bytes;
use tokio::io::{ReadHalf, WriteHalf};
use tokio_tungstenite::MaybeTlsStream;

use crate::net::TcpStream;

pub type TcpWriter = WriteHalf<MaybeTlsStream<TcpStream>>;
pub type TcpReader = ReadHalf<MaybeTlsStream<TcpStream>>;
pub type TcpMessageHandler = Arc<dyn Fn(&[u8]) + Send + Sync>;

/// Represents a command for the writer task.
#[derive(Debug)]
pub enum WriterCommand {
    /// Update the writer reference with a new one after reconnection.
    Update(TcpWriter, tokio::sync::oneshot::Sender<bool>),
    /// Send data to the server.
    Send(Bytes),
}
