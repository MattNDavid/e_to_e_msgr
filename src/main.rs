mod commands;
mod establish_websocket;
mod manage_keys;
mod auth_cli;
mod session_cli;
mod to_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn::std::error::Error>> {
    // Initialize the CLI for authentication
    let (tx, rx) = auth_cli::cli().await?;
    
    // Start the session CLI with the established WebSocket
    session_cli::cli((tx, rx)).await?;

    Ok(())

}





    

