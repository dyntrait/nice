

//! Socket types and type aliases.

use std::sync::Arc;

use bytes::Bytes;
use tokio::io::{ReadHalf, WriteHalf};
use tokio_tungstenite::MaybeTlsStream;

use crate::net::TcpStream;

/// The write half of a plain or TLS TCP stream.
pub type TcpWriter = WriteHalf<MaybeTlsStream<TcpStream>>;

/// The read half of a plain or TLS TCP stream.
pub type TcpReader = ReadHalf<MaybeTlsStream<TcpStream>>;

/// A thread-safe callback for complete suffix-framed messages.
pub type TcpMessageHandler = Arc<dyn Fn(&[u8]) + Send + Sync>;

/// A command processed by the socket writer task.
#[derive(Debug)]
pub enum WriterCommand<W = TcpWriter> {
    /// Replaces the writer after reconnection and reports whether buffered messages were drained.
    Update(W, tokio::sync::oneshot::Sender<bool>),
    /// Replaces the writer, sends reconnect replay first, then drains buffered messages.
    UpdateWithReplay(W, Vec<Bytes>, tokio::sync::oneshot::Sender<bool>),
    /// Sends data to the server.
    Send(Bytes),
    /// Sends data once, either directly or through reconnect replay.
    ///
    /// A newer buffered command with the same key supersedes an older one. If reconnect replay
    /// already contains the exact data, the buffered command is discarded after replay.
    SendOrReplay { key: u64, data: Bytes },
}
