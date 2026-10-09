

//! Raw TCP clients with suffix framing, optional TLS, heartbeats, and automatic reconnection.
//!
//! # Architecture
//!
//! [`SocketClient`] uses a controller to own connection lifecycle. One reader task splits the byte
//! stream into complete messages for [`TcpMessageHandler`], while one writer task serializes sends
//! from concurrent callers. Optional heartbeat traffic passes through the same writer.
//!
//! # Framing and liveness
//!
//! [`SocketConfig::suffix`] frames both directions. The writer appends it to application and
//! heartbeat messages, and the reader strips it before dispatch. An empty suffix is rejected. A
//! partial frame that exceeds 10 MiB stops the reader and triggers reconnect. An optional idle
//! timeout detects a connection that remains open without delivering bytes.
//!
//! # State reporting and explicit reconnect
//!
//! An optional [`crate::SocketStateSink`] publishes ordered `Connected` and `Disconnected`
//! availability edges for initial connection, transport loss, and recovery. It omits retry attempts
//! and deliberate shutdown. [`SocketReconnectHandle`] lets adapter tasks request transport
//! replacement without owning the client and reports whether each request was accepted, already
//! reconnecting, disconnecting, or closed.
//!
//! # Reconnection and replay
//!
//! Initial connection establishment retries failures with exponential backoff. When a connected
//! transport fails, the controller reconnects with configurable backoff, jitter, timeout,
//! and attempt limits. The writer buffers application messages in FIFO order. A
//! [`SocketReconnectReplay`] can place protocol setup messages before that buffer on the replacement
//! connection, and a post-reconnection callback runs after the writer, buffer, and reader are ready.
//!
//! # Transport policy
//!
//! Connections support plain TCP or `rustls`, enable `TCP_NODELAY`, and accept either a raw
//! `host:port` address or a URL. A certificate directory can add trusted roots and supply a matching
//! client certificate and key.

pub mod client;
pub mod config;
pub mod types;

pub use client::{SocketClient, SocketReconnectHandle, SocketReconnectReplay};
pub use config::{SocketConfig, SocketHeartbeat};
pub use types::{TcpMessageHandler, TcpReader, TcpWriter, WriterCommand};
