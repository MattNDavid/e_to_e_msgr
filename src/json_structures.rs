use serde::{Deserialize};
use serde_json;

#[derive(Deserialize)]
pub struct Devices {
    pub user_id: String,
    pub devices: Vec<serde_json::Value>,
    pub timestamp: String
}

struct Message {
    _type: String, 
    sender: String,
    content: String,
    timestamp: String
}