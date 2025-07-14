use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;

use crate::commands;

/**
 * A simple CLI for session management (sending/receiving messages, etc.).
 * NOT INTENDED FOR PRODUCTION USE.
 */
pub async fn cli((tx, rx): (SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>, SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>) ) -> Result<(), Box<dyn std::error::Error>> {
    let _ = commands::begin_recv_task(rx);
    
    Ok(())
}
