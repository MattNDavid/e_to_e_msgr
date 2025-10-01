/**
 * The end-to-end encryption process requires a device table that connects device to users
 */
use tokio_rusqlite::Connection;

pub async fn initialize_db(user_id: &str) -> Result<Connection, Box<dyn std::error::Error + Send + Sync>> {
    let db_name = format!("{}.database", user_id);

    let conn = Connection::open(db_name).await?;
    let users =  
    "CREATE TABLE users (
        user_id TEXT PRIMARY KEY,
        email TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );";
    let devices = 
    "CREATE TABLE devices (
        device_id INTEGER PRIMARY KEY,
        user_id TEXT NOT NULL UNIQUE,
        shared_key TEXT,
        msg_sequence_num INTEGER NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users(user_id)
    );";
    let conversations = 
    "CREATE TABLE conversations (
        conversation_id INTEGER PRIMARY KEY,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        last_active TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );";
    let user_conversations = 
    "CREATE TABLE user_conversations (
        user_id TEXT NOT NULL,
        conversation_id INTEGER NOT NULL,
        PRIMARY KEY (user_id, conversation_id),
        FOREIGN KEY (user_id) REFERENCES users(user_id),
        FOREIGN KEY (conversation_id) REFERENCES conversations(conversation_id)
    );";
    let messages = 
    "CREATE TABLE messages (
        message_id INTEGER PRIMARY KEY,
        conversation_id INTEGER NOT NULL,
        sender_id TEXT NOT NULL,
        content TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        received_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (conversation_id) REFERENCES conversations(conversation_id),
        FOREIGN KEY (sender_id) REFERENCES users(user_id)
    );";

    conn.call(move |call| {
        //Remove newlines from query so that sqlite accepts them
        call.execute(users,  []).map_err(tokio_rusqlite::Error::from)?;
        call.execute(devices,  []).map_err(tokio_rusqlite::Error::from)?;
        call.execute(conversations,  []).map_err(tokio_rusqlite::Error::from)?;
        call.execute(user_conversations,  []).map_err(tokio_rusqlite::Error::from)?;
        call.execute(messages,  []).map_err(tokio_rusqlite::Error::from)?;

        Ok(())
    }).await?;
    Ok(conn)
}

pub async fn connect(user_id: &str) -> Result<Connection, Box<dyn std::error::Error + Send + Sync>> {
    let db_name = format!("{}.database", user_id);
    let conn = Connection::open(db_name).await?;
    Ok(conn)
}