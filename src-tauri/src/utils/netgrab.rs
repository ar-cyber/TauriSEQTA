use reqwest;
use std::collections::HashMap;


#[tauri::command]
pub async fn grab_api_data(url: &str) -> Result<HashMap<String, String>, String> {
    match reqwest::get("https://httpbin.org/ip").await {
        Ok(resp) => {
            match resp.json::<HashMap<String, String>>().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
        }
        Err(e) => Err(format!("HTTP request failed: {}", e)),
    }
}