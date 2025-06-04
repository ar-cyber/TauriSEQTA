use reqwest;
use reqwest::Client;
use rss::Channel;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
// src-tauri/src/main.rs
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::OpenerExt; // Import the trait to use .opener()


// opens a file using the default program:

#[path = "../utils/session.rs"]
mod session;

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

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMethod {
    GET,
    POST,
}

/// Build an HTTP client with headers based on the saved session.
fn create_client() -> reqwest::Client {
    let session = session::Session::load();
    let mut headers = reqwest::header::HeaderMap::new();

    // Build the complete cookie string with JSESSIONID and additional cookies
    let mut cookie_parts = Vec::new();

    // Add JSESSIONID first if it exists
    if !session.jsessionid.is_empty() {
        cookie_parts.push(format!("JSESSIONID={}", session.jsessionid));
    }

    // Add all additional cookies
    for cookie in session.additional_cookies {
        cookie_parts.push(format!("{}={}", cookie.name, cookie.value));
    }

    // Set the combined cookie header if we have any cookies
    if !cookie_parts.is_empty() {
        headers.insert(
            reqwest::header::COOKIE,
            cookie_parts.join("; ").parse().unwrap(),
        );
    }

    headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (DesQTA)".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        "en-US,en;q=0.9".parse().unwrap(),
    );

    if !session.base_url.is_empty() {
        headers.insert(reqwest::header::ORIGIN, session.base_url.parse().unwrap());
        headers.insert(reqwest::header::REFERER, session.base_url.parse().unwrap());
    }

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to create HTTP client")
}

#[tauri::command]
pub async fn fetch_api_data(
    url: &str,
    method: RequestMethod,
    headers: Option<HashMap<String, String>>,
    body: Option<Value>,
    parameters: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let client = create_client();
    let session = session::Session::load();
    let full_url = if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{}{}", session.base_url.parse::<String>().unwrap(), url)
    };

    let mut request = match method {
        RequestMethod::GET => client.get(&full_url),
        RequestMethod::POST => client.post(&full_url),
    };

    // Add custom headers if provided
    if let Some(headers) = headers {
        for (key, value) in headers {
            request = request.header(&key, value);
        }
    }

    // Add query parameters if provided
    if let Some(params) = parameters {
        request = request.query(&params);
    }

    // Add body for POST requests if provided
    if let RequestMethod::POST = method {
        if let Some(body_data) = body {
            request = request.json(&body_data);
        }
    }

    match request.send().await {
        Ok(resp) => {
            let response = resp.text().await.unwrap();
            Ok(response)
        }
        Err(e) => Err(format!("HTTP request failed: {e}")),
    }
}

#[tauri::command]
pub async fn get_api_data(
    url: &str,
    parameters: HashMap<String, String>,
) -> Result<String, String> {
    fetch_api_data(url, RequestMethod::GET, None, None, Some(parameters)).await
}

#[derive(Serialize)]
pub struct FeedItem {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    pub_date: Option<String>,
}

#[tauri::command]
pub async fn get_rss_feed(feed: &str) -> Result<Vec<FeedItem>, String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let res = client
        .get(feed)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = res.status();
    let content = res
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Request failed with status {}. Response body:\n{}",
            status, content
        ));
    }

    let channel = Channel::read_from(content.as_bytes())
        .map_err(|e| format!("Failed to parse RSS feed: {}", e))?;

    let items: Vec<FeedItem> = channel
        .items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().map(|s| s.to_string()),
            link: item.link().map(|s| s.to_string()),
            description: item.description().map(|s| s.to_string()),
            pub_date: item.pub_date().map(|s| s.to_string()),
        })
        .collect();

    Ok(items)
}



#[tauri::command]
pub async fn post_api_data(
    url: &str,
    data: Value,
    parameters: HashMap<String, String>,
) -> Result<String, String> {
    fetch_api_data(url, RequestMethod::POST, None, Some(data), Some(parameters)).await
}
