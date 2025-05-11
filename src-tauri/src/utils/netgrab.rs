use reqwest;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

const JSESSIONID: &str = "token here";
const BASE_URL: &str = "https://domain.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeworkItem {
    pub meta: i32,
    pub id: i32,
    pub title: String,
    pub items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeworkResponse {
    pub payload: Vec<HomeworkItem>,
    pub status: String,
}

// This function provides a method to make GET requests to a specified URL with optional parameters.
// It takes a URL and a Hashmap of parameters as an input, and returns a result containing response data or an error.
fn create_client() -> reqwest::Client {
    reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::COOKIE,
                format!("JSESSIONID={}", JSESSIONID).parse().unwrap()
            );
            headers.insert(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36".parse().unwrap()
            );
            headers.insert(
                reqwest::header::ACCEPT,
                "application/json, text/plain, */*".parse().unwrap()
            );
            headers.insert(
                reqwest::header::ACCEPT_LANGUAGE,
                "en-US,en;q=0.9".parse().unwrap()
            );
            headers.insert(
                reqwest::header::ORIGIN,
                BASE_URL.parse().unwrap()
            );
            headers.insert(
                reqwest::header::REFERER,
                BASE_URL.parse().unwrap()
            );
            headers
        })
        .build()
        .expect("Failed to create HTTP client")
    }

// This function provides a method to make GET requests to a specified URL with optional parameters.
// It takes a URL and a Hashmap of parameters as an input, and returns a result containing response data or an error.
#[tauri::command]
pub async fn get_api_data(url: &str, parameters: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
    let client = reqwest::Client::new();
    match client.get(url)
    .query(&parameters) // Apply parameters
    .send() // Send
    .await {
        Ok(response) => {
            match response.json::<HashMap<String, String>>().await { // Check if its json
                Ok(data) => Ok(data), // return the json data
                Err(e) => Err(format!("Failed to parse JSON: {}", e)), // otherwise error out (might change in the future)
            }
        },
        Err(e) => Err(format!("HTTP request failed: {}", e)) // Something happened with the request, bailing out
    }
}

// This function provides a method to make POST requests to a specified URL with a JSON body, specified through a Hashmap.
// JSON Body is specified through a HashMap containing key value pairs
// Result is the response data or an error.
#[tauri::command]
pub async fn post_api_data(url: &str, data: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
    let client = reqwest::Client::new();
    match client.get(url)
    .json(&data) // Add the HashMap as JSON body
    .send()
    .await {
        Ok(response) => {
            match response.json::<HashMap<String, String>>().await { // Check if it's json
                Ok(data) => Ok(data), // return the json data
                Err(e) => Err(format!("Failed to parse JSON: {}", e)), // otherwise error out (might change in the future)
            }
        },
        Err(e) => Err(format!("HTTP request failed: {}", e)) // Bailing out because something happened in HTTP
    }
}