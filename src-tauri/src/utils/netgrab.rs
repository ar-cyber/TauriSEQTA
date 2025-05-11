use reqwest;
use std::collections::HashMap;


#[tauri::command]
pub async fn get_api_data(url: &str, parameters: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
    let client = reqwest::Client::new();
    match client.get(url)
    .query(&parameters)
    .send()
    .await {
        Ok(response) => {
            match response.json::<HashMap<String, String>>().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
        },
        Err(e) => Err(format!("HTTP request failed: {}", e))
    }
}

#[tauri::command]
pub async fn post_api_data(url: &str, data: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
    let client = reqwest::Client::new();
    match client.get(url)
    .json(&data)
    .send()
    .await {
        Ok(response) => {
            match response.json::<HashMap<String, String>>().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
        },
        Err(e) => Err(format!("HTTP request failed: {}", e))
    }
}