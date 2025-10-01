use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::{header::HeaderValue, HeaderMap};
use futures_util::{StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use serde_json::json;

use crate::manage_keys;
use crate::session_manager::process_message;

pub async fn establish_websocket(username: &str) -> Result<
    (
        SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
    ),
    Box<dyn std::error::Error + Send + Sync>
> {
    let url = "ws://localhost:3000/ws";
    let mut request = url.into_client_request()?;

    let headers = request.headers_mut();
    headers.insert("x-user-id", HeaderValue::from_str(&username)?);
    headers.insert("x-auth-token", HeaderValue::from_str(&manage_keys::get_token(username).await?)?);
    headers.insert("x-device-id", HeaderValue::from_str(&manage_keys::get_device_id(username).await?)?);
    headers.insert("x-device-uuid", HeaderValue::from_str(&manage_keys::get_uuid(username).await?)?);

    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect to WebSocket");
    let (send, mut recv) = ws_stream.split();

    // receive the message with the new token.
    let initial_message = recv.next().await.unwrap()?.to_string();
    process_message(&initial_message).await?;

    Ok((send, recv))
}


