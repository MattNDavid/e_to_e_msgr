use serde_json::{Value, json};

pub async fn message(username: &str, recipient: &str, content: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let payload = json!({
        "type": "message",
        "sender": username,
        "recipient": recipient,
        "content": content
    });

    Ok(payload)
}

pub async fn get_devices(username: &str) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let payload = json!({
        "type": "user",
        "action": "get_devices",
        "user_id": username
    });

    Ok(payload)
}