use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
use futures_util::stream::{SplitStream};
use futures_util::{StreamExt, SinkExt};


mod auth_commands;
mod establish_websocket;
mod manage_keys;
mod auth_cli;
mod session_cli;
mod to_server;
mod session_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn::std::error::Error + Send + Sync>> {
    // Initialize the CLI for authentication
    let (tx, rx) = auth_cli::cli().await?;
    session_manager::session(tx, rx).await?;


    Ok(())

}





    

