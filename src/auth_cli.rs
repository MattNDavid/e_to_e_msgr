use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
use futures_util::stream::{SplitStream, SplitSink};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::io::{AsyncWriteExt};
use csv::ReaderBuilder;
use tokio::net::TcpStream;

use crate::auth_commands;
/**
 * A simple CLI for loggin in with the messenger client.
 * NOT INTEDED FOR PRODUCTION USE.
 */

pub async fn cli() ->Result<
    (
        SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
    ), Box<dyn std::error::Error + Send + Sync>> {

    let mut stdout = io::stdout();
    stdout.write_all(b"Welcome to the End-to-End Encrypted Messenger CLI!\n1. New Account\n2. Login\n").await?;
    stdout.flush().await?;

    let mut input = String::new();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut input).await?;

    match input.trim() {
        "1" => {
            // Call new_account function to collect user input and send request to server
            match new_account().await {
                Ok((send, recv)) => {
                    println!("Account created successfully , logging in...");
                    Ok((send, recv))
                }
                Err(e) => {
                    eprintln!("Failed to create account: {}", e);
                    Err(e)
                }
            }
        }
        "2" => {
            // Call login function
            match login().await {
                Ok((send, recv)) => {
                    println!("Logged in successfully!");
                    Ok((send, recv))
                }
                Err(e) => {
                    eprintln!("Failed to login: {}", e);
                    Err(e)
                }
            }
        }
        _ => {
            eprintln!("Invalid option");
            Err(Box::from("Invalid option selected"))
        }
    }
}
/*
Takes user input for username, email, and password, generates a UUID (if it doesn't exist), and sends a request
to the server to create a new account.
Eventually, should return a websocket connection to the server for further communication.
*/
async fn new_account() -> Result<
    (
        SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
    ), Box<dyn std::error::Error + Send + Sync>> {
    // Implement the logic for creating a new account
    let mut stdout = io::stdout();
    stdout.write_all(b"Enter username, email, and password in format 'username,email,password': ").await?;
    stdout.flush().await?;

    let mut input = String::new();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut input).await?;

    //split input by commas
    let parts: Vec<&str> = input.trim().split(',').collect();
    if parts.len() != 3 {
        eprintln!("Invalid input format. Please provide username, email, and password separated by commas.");
        return Err(Box::from("Invalid input format"));
    }
    let username = parts[0].trim();
    let email = parts[1].trim();
    let password = parts[2].trim();

    Ok(auth_commands::new_account(username, email, password).await?)
}

async fn login() -> Result<
(
    SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
    SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
), Box<dyn std::error::Error + Send + Sync>> {
    //check for previously logged in users
    let mut stdout = io::stdout();
    
    // Check if file exists first
    //If not, present options to login with new account or quit
    if tokio::fs::metadata("users.csv").await.is_err() {
        stdout.write_all(b"No previous users found.\n1. Login with new account\n2. Quit\nSelect option: ").await?;
        stdout.flush().await?;
        
        let mut input = String::new();
        let mut reader = BufReader::new(io::stdin());
        reader.read_line(&mut input).await?;
        let input = input.trim();
        if input == "1" {
            return Ok(login_new().await?);
        } else if input == "2" {
            return Err(Box::from("User chose to quit"));
        } else {
            eprintln!("Invalid option selected.");
            return Err(Box::from("Invalid option selected"));
        }
        
    }
    //if users.csv exists...
    let mut readcsv = ReaderBuilder::new()
        .has_headers(false)
        .from_path("users.csv")?;
    let mut option_number = 0;
    let mut records: Vec<String> = Vec::new();

    //print options for existing users
    for result in readcsv.records() {
        option_number += 1;
        let record = result?;
        if let Some(username) = record.get(0) {
            let username = username.trim();
            stdout.write_all(format!("{}. {}\n", option_number, username).as_bytes()).await?;
            stdout.flush().await?;
            records.push(username.to_string());
        }
    }
    //print new account option
    option_number += 1;
    stdout.write_all(format!("{}. Login with new account\nSelect option from above: ", option_number).as_bytes()).await?;
    stdout.flush().await?;
    
    //read user input for option selection
    let mut input = String::new();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut input).await?;
    let input = input.trim();
    let selected_option = input.parse::<usize>().unwrap_or(0);

    if selected_option == option_number {
        return Ok(login_new().await?);
    } else if selected_option > 0 && selected_option <= records.len() {
        let username = &records[selected_option - 1];
        return Ok(login_existing(username).await?);
    } else {
        eprintln!("Invalid option selected.");
        return Err(Box::from("Invalid option selected"));
    }
}

async fn login_existing(username: &str) -> Result<
    (
        SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
    ), Box<dyn std::error::Error + Send + Sync>> {
    Ok(auth_commands::login_existing(username).await?)
}

async fn login_new() -> Result<
    (
        SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>
    ), Box<dyn std::error::Error + Send + Sync>> {
    let mut stdout = io::stdout();
    stdout.write_all(b"Enter username and password in format 'username,password': ").await?;
    stdout.flush().await?;

    let mut input = String::new();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut input).await?;

    //split input by commas
    let parts: Vec<&str> = input.trim().split(',').collect();
    if parts.len() != 3 {
        eprintln!("Invalid input format. Please provide username, email, and password separated by commas.");
        return Err(Box::from("Invalid input format"));
    }
    let username = parts[0].trim();
    let password = parts[1].trim();

    Ok(auth_commands::login_new(username, password).await?)
}







