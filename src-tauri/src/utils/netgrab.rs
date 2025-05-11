use reqwest;
use std::collections::HashMap;

const JSESSIONID: &str = "";
const BASE_URL: &str = "";


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
pub async fn get_api_data(url: &str, parameters: HashMap<String, String>) -> Result<String, String> {
    let client = create_client(); // Create a new HTTP client with custom headers
    let full_url = if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{}{}", BASE_URL, url)
    };
    match client.get(&full_url) // Create a GET request with the URL
    .query(&parameters) // Apply parameters
    .send() // Send
    .await {
        Ok(response) => Ok(format!("{}", response.text().await.unwrap_or_else(|_| "Failed to read response".to_string()))), // Return the response text
        Err(e) => Err(format!("HTTP request failed: {}", e)) // Something happened with the request, bailing out
    }
}

// This function provides a method to make POST requests to a specified URL with a JSON body, specified through a Hashmap.
// JSON Body is specified through a HashMap containing key value pairs
// Result is the response data or an error.
#[tauri::command]
pub async fn post_api_data(url: &str, data: HashMap<String, String>, parameters: HashMap<String, String>) -> Result<String, String> {
    let client = create_client(); // Create a new HTTP client with custom headers

    let full_url = if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{}{}", BASE_URL, url)
    };

    match client.post(&full_url)
    .json(&data) // Add the HashMap as JSON body
    .query(&parameters) // Apply parameters
    .send()
    .await {
        Ok(response) => Ok(format!("{}", response.text().await.unwrap_or_else(|_| "Failed to read response".to_string()))), // Return the response text
        Err(e) => Err(format!("HTTP request failed: {}", e)) // Bailing out because something happened in HTTP
    }
}