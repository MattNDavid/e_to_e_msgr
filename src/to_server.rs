use reqwest::Client;

//only for POST atp
pub async fn to_server(uri: &str, payload: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("http://localhost:3000/{}", uri);
    let client = Client::new();
    let resp = client.post(url)
        .json(&payload)
        .send()
        .await?;

    if resp.status().is_success() {
        let json_response: serde_json::Value = resp.json().await?;
        Ok(json_response)
    } else {
        Err(Box::from("Failed to send HTTP request"))
    }
}