use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
use futures_util::stream::{SplitStream, SplitSink};
use futures_util::{StreamExt, SinkExt};
use tokio::io::{AsyncBufReadExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use crate::manage_keys::store_token;

pub async fn session(
    tx: SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
    rx: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (msg_tx, msg_rx) = mpsc::channel::<String>(32);

    //Spawn a task for receiving messages
    let rx_handle = tokio::spawn(rx_task(rx));
    // Spawn a task for sending messages
    let tx_handle = tokio::spawn(tx_task(tx, msg_rx));
    //Use main task to manage input

    // Await both tasks and propagate any errors
    let _rx_result = rx_handle.await?;
    let _tx_result = tx_handle.await?;

    Ok(())
}

async fn rx_task(
    mut rx: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    while let Some(msg) = rx.next().await {
        match msg {
            Ok(Message::Text(text)) => {

                println!("Received: {}", text);
                process_message(&text.to_string()).await?;
                
            }
            Ok(Message::Binary(_)) => {
                println!("Received binary message");
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}


async fn tx_task(
    mut tx: SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>, mut msg_rx: mpsc::Receiver<String>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    while let Some(msg) = msg_rx.recv().await {
        tx.send(Message::Text(msg.into())).await?;
    }

    Ok(())
}

async fn process_message(msg: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let msg: serde_json::Value = serde_json::from_str(msg)?;

    let msg_type = msg.get("type")
        .and_then(|v| v.as_str())
        .ok_or("Message type not found")?;

    match msg_type {
        "auth" => {
            auth_handler(msg).await?;
        }
        _ => {
            return Err(Box::from("Unknown message type"));
        }
    }


    Ok(())
}

async fn auth_handler(msg: serde_json::Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subtype = msg.get("subtype")
        .and_then(|v| v.as_str())
        .ok_or("Auth subtype not found")?;
    match subtype {
        "confirm" => {
            // Handle confirmation
            println!("Handling confirmation");
            let result = auth_confirm(msg).await;
            println!("Confirmation handled");
            return result;
        }
        "logout" => {
            // Handle logout
            println!("Handling logout");
        }
        _ => {
            return Err(Box::from("Unknown auth subtype"));
        }
    }

    Ok(())
}

pub async fn auth_confirm(msg: serde_json::Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let token = msg.get("token")
        .and_then(|v| v.as_str())
        .ok_or("Token not found")?;

    println!("Token: {}", token);

    let user_id = msg.get("user_id")
        .and_then(|v| v.as_str())
        .ok_or("User ID not found")?;

    store_token(token, user_id).await?;

    Ok(())
}
