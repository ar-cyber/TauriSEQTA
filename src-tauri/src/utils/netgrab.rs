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
pub async fn get_api_data(url: &str, parameters: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
    println!("GET request to URL: {}", url);
    println!("Parameters: {:?}", parameters);
    
    let client = create_client();
    
    // Construct the full URL if it's a relative path
    let full_url = if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{}{}", BASE_URL, url)
    };
    println!("Full URL: {}", full_url);

    match client.get(&full_url)
        .query(&parameters)
        .send()
        .await {
            Ok(response) => {
                println!("Response status: {}", response.status());
                println!("Response headers: {:?}", response.headers());
                
                let text = response.text().await.map_err(|e| format!("Failed to get response text: {}", e))?;
                println!("Response body: {}", text);
                
                match serde_json::from_str::<HashMap<String, String>>(&text) {
                    Ok(data) => Ok(data),
                    Err(e) => Err(format!("Failed to parse JSON: {}. Raw response: {}", e, text)),
                }
            },
            Err(e) => Err(format!("HTTP request failed: {}", e))
        }
}

// This function provides a method to make POST requests to a specified URL with a JSON body, specified through a Hashmap.
// JSON Body is specified through a HashMap containing key value pairs
// Result is the response data or an error.
#[tauri::command]
pub async fn post_api_data(url: &str, data: HashMap<String, String>) -> Result<HomeworkResponse, String> {
    println!("POST request to URL: {}", url);
    println!("Request data: {:?}", data);
    
    let client = create_client();
    
    // Construct the full URL if it's a relative path
    let full_url = if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{}{}", BASE_URL, url)
    };
    println!("Full URL: {}", full_url);

    match client.post(&full_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await {
            Ok(response) => {
                println!("Response status: {}", response.status());
                println!("Response headers: {:?}", response.headers());
                
                let text = response.text().await.map_err(|e| format!("Failed to get response text: {}", e))?;
                println!("Response body: {}", text);
                
                match serde_json::from_str::<HomeworkResponse>(&text) {
                    Ok(data) => Ok(data),
                    Err(e) => Err(format!("Failed to parse JSON: {}. Raw response: {}", e, text)),
                }
            },
            Err(e) => Err(format!("HTTP request failed: {}", e))
        }

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