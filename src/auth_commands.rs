use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use csv::ReaderBuilder;
use serde_json::{de, json};
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{StreamExt};
use tokio::net::TcpStream;


use crate::manage_keys;
use crate::establish_websocket;
use crate::to_server;

pub async fn new_account(username: &str, email: &str, password: &str) -> Result<
    (
        SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
    ), Box<dyn std::error::Error + Send + Sync>> {
    //create uuid for the new account
    let dev_id = match manage_keys::get_uuid(username).await {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Failed to retrieve device UUID. Generating a new one.");
            manage_keys::generate_uuid(username).await.unwrap()
        }
    };

    let request = json!({
        "type": "new_account",
        "username": username,
        "email": email,
        "password": password,
        "uuid": dev_id
    });

    let resp = to_server::to_server("new_account", request).await?;
    let token = resp.get("token").and_then(|t| t.as_str()).unwrap_or("");
    let device_id = resp.get("device_id").and_then(|d| d.as_str()).unwrap_or("");
    //store token & device_id securely in WCM for future auth
    manage_keys::store_token(token, username).await?;
    manage_keys::store_device_id(username, device_id).await?;
    manage_keys::store_uuid(username, &dev_id).await?;
    //store the user in a local file for future login
    store_user(username).await?;

    Ok(login_existing(username).await?)
}

pub async fn login_new(username: &str, password: &str) -> Result<
    (
        SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
    ), Box<dyn std::error::Error + Send + Sync>> {

    if check_for_user(username).await? {
        
        return Ok(login_existing(username).await?);
    }

    let dev_id = match manage_keys::get_uuid(username).await {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Failed to retrieve device UUID. Generating a new one.");
            manage_keys::generate_uuid(username).await.unwrap()
        }
    };

    let resp = to_server::to_server("authenticate", json!({
        "username": username,
        "password": password,
        "uuid": dev_id
    })).await?;

    //store token & device id securely in WCM for future auth
    manage_keys::store_token(resp.get("token").unwrap().as_str().unwrap(), username).await?;
    manage_keys::store_device_id(username, resp.get("device_id").unwrap().as_str().unwrap()).await?;
    manage_keys::store_uuid(username, &dev_id).await?;
    //store the user in a local file for future login
    store_user(username).await?;

    Ok(login_existing(username).await?)
}

pub async fn login_existing(username: &str) -> Result<
    (
        SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
    ), Box<dyn std::error::Error + Send + Sync>> {

    Ok(establish_websocket::establish_websocket(username).await?)
}


async fn store_user(username: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("users.csv")
        .await?;
    let content = format!("{}\n", username);
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

async fn check_for_user(username: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)  // Add this since you don't have headers
        .from_path("users.csv")?;
    
    for result in reader.records() {
        let record = result?;
        if let Some(stored_username) = record.get(0) {
            if stored_username.trim() == username.trim() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}