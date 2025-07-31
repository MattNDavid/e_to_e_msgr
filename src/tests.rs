/**
 * Use only in development environment.
 * Will saturate server-side db with test data.
 * 
 */
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{SinkExt};

use crate::auth_commands;
use crate::manage_keys;
use crate::messages;

pub async fn run_all_tests() {
    // Run all tests sequentially
    new_account_valid().await;
    store_uuid_test().await;
    delete_credential_test().await;
    send_message_test().await;

    // Add more tests as needed
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
pub async fn new_account_valid() {
    let result = auth_commands::new_account("test", "test@example.com", "password123").await;
    assert!(result.is_ok(), "new_account_valid failed: {:?}", result.err());
    println!("New Account Test Passed");
    // Clean up by deleting the test user credentials
    /*
    manage_keys::delete_credential("test", "e_to_e_msgr_uuid").await.unwrap();
    manage_keys::delete_credential("test", "e_to_e_msgr_token").await.unwrap();
    manage_keys::delete_credential("test", "e_to_e_msgr_device_id").await.unwrap();
    */
    }
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
    let device_key = "test-device-key";

    let result = manage_keys::delete_credential(username, "e_to_e_msgr_uuid").await;
    assert!(result.is_ok(), "delete_credential_test failed: {:?}", result.err());
    println!("Delete Credential Test Passed");
}
pub async fn send_message_test() {
    let (mut tx, rx) = auth_commands::login_existing("test").await.unwrap();
    let message = messages::message("test", "recipient", "Hello, world!").await.unwrap();
    tx.send(Message::Text(message.to_string().into())).await.unwrap();

}