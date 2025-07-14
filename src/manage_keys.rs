use uuid::Uuid;
use keyring::Entry;

pub async fn generate_uuid(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let device_key = Uuid::new_v4().to_string();

    Ok(device_key)
}

pub async fn store_uuid(username: &str, device_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keyring = Entry::new("e_to_e_msgr_uuid", username)?;
    keyring.set_password(device_key)?;

    Ok(())
}

pub async fn get_uuid(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let keyring = Entry::new("e_to_e_msgr_uuid", username)?;

    match keyring.get_password() {
        Ok(device_key) => Ok(device_key),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn store_token(token: &str, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keyring = Entry::new("e_to_e_msgr_token", username)?;
    keyring.set_password(token)?;

    Ok(())
}

pub async fn get_token(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let keyring = Entry::new("e_to_e_msgr_token", username)?;

    match keyring.get_password() {
        Ok(token) => Ok(token),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn store_device_id(username: &str, device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keyring = Entry::new("e_to_e_msgr_device_id", username)?;
    keyring.set_password(device_id)?;

    Ok(())
}

pub async fn get_device_id(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let keyring = Entry::new("e_to_e_msgr_device_id", username)?;

    match keyring.get_password() {
        Ok(device_id) => Ok(device_id),
        Err(e) => Err(Box::new(e)),
    }
}