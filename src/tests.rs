/**
 * Use only in development environment.
 * Will saturate server-side db with test data.
 */
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use tokio::fs;
use tokio_rusqlite;

use crate::auth_commands;
use crate::manage_keys;
use crate::messages;
use crate::json_structures;
use crate::db;


pub async fn run_all_tests() {

    create_db().await.unwrap();

    new_account_valid().await;
    add_necessary_accounts().await;
    store_uuid_test().await;
    delete_credential_test().await;
    send_message().await;
    get_devices().await;
    store_devices().await;
}
/*
AUTH COMMANDS TESTS
*/
//Basic new account test using valid inputs
//Need to clear account from server side db before running this test again
/**
 * Also tests:
 * - UUID generation
 * - Token and device ID storage
 * - Establish WebSocket connection
 * - Login Existing user (new account uses the new_account server endpoint to create the account, then 
 * uses login_existing to establish a WebSocket connection using the session credentials from the initial http response)
 */
pub async fn create_db() -> Result<(), Box<dyn std::error::Error>> {
    let db = db::initialize_db("testing").await.map_err(|e| {
        println!("Database initialization failed: {:?}", e);
    }).unwrap();
    let conn = db.call(|call| {
        let mut stmt = call.prepare("SELECT name FROM sqlite_master WHERE type='table';")?;
        let mut rows = stmt.query([])?;
        let mut table_names = Vec::new();
        while let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            table_names.push(name);
        }
        Ok(table_names)
    }).await.unwrap();

    assert!(conn.contains(&"users".to_string()), "Users table not found");
    assert!(conn.contains(&"devices".to_string()), "Devices table not found");
    assert!(conn.contains(&"conversations".to_string()), "Conversations table not found");
    assert!(conn.contains(&"user_conversations".to_string()), "User_Conversations table not found");
    assert!(conn.contains(&"messages".to_string()), "Messages table not found");

    db.close().await.unwrap();

    fs::remove_file("testing.database").await?;
    
    Ok(())
}
pub async fn new_account_valid() {
    if manage_keys::get_uuid("test").await.is_ok() {
        //Account can only exist in this environment if this test has previously run and passed
        return;    
    }
    let result = auth_commands::new_account("test", "test@example.com", "password123").await;
    assert!(result.is_ok(), "new_account_valid failed: {:?}", result.err());
    println!("New Account Test Passed");
}
pub async fn add_necessary_accounts() {
    if manage_keys::get_uuid("example").await.is_err() {
        let _ = auth_commands::new_account("example", "example@test.com", "password123").await.unwrap();
    }
    if manage_keys::get_uuid("test").await.is_err() {
        let _ = auth_commands::new_account("test", "test@example.com", "password123").await.unwrap();
    }
}
/*
pub async fn auth_new_device() {

    let result = auth_commands::login_new("test", "password123").await;
    assert!(result.is_ok(), "auth_new_device failed: {:?}", result.err());

    println!("auth_new_device test passed");
}
*/
/*
MANAGE KEYS TESTS
*/
pub async fn store_uuid_test() {
    let username = "test_uuid";
    let device_key = "test-device-key";
    let result = manage_keys::store_uuid(username, device_key).await;

    assert!(result.is_ok(), "store_uuid_test failed: {:?}", result.err());
    println!("Store UUID Test Passed");
}
pub async fn delete_credential_test() {
    let username = "test_uuid";

    let result = manage_keys::delete_credential(username, "e_to_e_msgr_uuid").await;
    assert!(result.is_ok(), "delete_credential_test failed: {:?}", result.err());

    println!("Delete Credential Test Passed");
}
/*
MESSAGE TESTS
*/

//should result in the message being received by the server and echoed back to this client
pub async fn send_message() {
    let (mut _tx1, mut rx1) = auth_commands::login_existing("example").await.unwrap();
    let (mut tx, mut _rx) = auth_commands::login_existing("test").await.unwrap();
    
    let message = messages::message("test", &manage_keys::get_device_id("example").await.unwrap(), "Hello, world!").await.unwrap();
    
    tx.send(Message::Text(message.to_string().into())).await.unwrap();
    
    let received = rx1.next().await.unwrap().unwrap();

    let json = serde_json::from_str::<serde_json::Value>(&received.to_string()).unwrap();

    assert_eq!(json["content"], "Hello, world!");
    assert_eq!(json["sender"], manage_keys::get_device_id("test").await.unwrap());

    println!("Send message test passed");
}

pub async fn get_devices() {
    let (mut tx, mut rx) = auth_commands::login_existing("test").await.unwrap();

    let message = messages::get_devices("test").await.unwrap();
    tx.send(Message::Text(message.to_string().into())).await.unwrap();

    let received = rx.next().await.unwrap().unwrap();
    let raw_json: serde_json::Value = serde_json::from_str(&received.to_string()).unwrap();

    let json = json_structures::Devices {
        user_id: raw_json.get("user_id").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default(),
        devices: raw_json.get("devices").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
        timestamp: raw_json.get("timestamp").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default(),
    };


    assert_eq!(json.devices[0].as_i64().unwrap() as i32, manage_keys::get_device_id("test").await.unwrap().parse::<i32>().unwrap());

    println!("get devices test passed");
}
pub async fn store_user() {
    let conn = db::connect("test").await.unwrap();
    let user_id = "test_user";
    let email = "someone@example.com";
    conn.call(move |call| {
        call.execute(
            "INSERT OR IGNORE INTO users (user_id, email) VALUES (?1, ?2)",
            &[&user_id, &email],
        ).map_err(tokio_rusqlite::Error::from)?;
        Ok(())
    }).await.unwrap();

    let exists = conn.call(move |call| {
        let mut stmt = call.prepare("SELECT COUNT(*) FROM users WHERE user_id = ?1")?;
        let mut rows = stmt.query([&user_id])?;
        if let Some(row) = rows.next()? {
            let count: i32 = row.get(0)?;
            Ok(count > 0)
        } else {
            Ok(false)
        }
    }).await.unwrap();

    assert!(exists, "User was not stored correctly in the database");

    println!("Store user test passed");
}
pub async fn store_devices() {
    let conn = db::connect("test").await.unwrap();
    let user_id = "test_user";
    let shared_key = "shared_key_example";
    let device_id = 1;
    let msg_sequence_num = 0;

    conn.call(move |call| {
        call.execute(
            "INSERT OR IGNORE INTO devices (device_id, user_id, shared_key, msg_sequence_num) VALUES (?1, ?2, ?3, ?4)",
            &[&device_id, &user_id, &shared_key, &msg_sequence_num],
        ).map_err(tokio_rusqlite::Error::from)?;
        Ok(())
    }).await.unwrap();

}
pub async fn new_conversation() {

}


