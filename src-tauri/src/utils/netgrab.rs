use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::session::Session;

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

/// Build an HTTP client with headers based on the saved session.
fn create_client() -> reqwest::Client {
    let session = Session::load();
    let mut headers = reqwest::header::HeaderMap::new();

    if !session.jsessionid.is_empty() {
        headers.insert(
            reqwest::header::COOKIE,
            format!("JSESSIONID={}", session.jsessionid).parse().unwrap(),
        );
    }

    headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (TauriSEQTA)".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert(reqwest::header::ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());

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
pub async fn get_api_data(
    url: &str,
    parameters: HashMap<String, String>,
) -> Result<String, String> {
    let client = create_client(); // Create a new HTTP client with custom headers
    
    let session = Session::load();
    
    let full_url = if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{}{}", session.base_url.parse::<String>().unwrap(), url)
    };

    match client.get(full_url).query(&parameters).send().await {
        Ok(resp) => Ok(format!("{}", resp.text().await.unwrap())),
        Err(e) => Err(format!("HTTP request failed: {e}")),
    }
}

#[tauri::command]
pub async fn post_api_data(
    url: &str,
    data: HashMap<String, String>,
    parameters: HashMap<String, String>
) -> Result<String, String> {
    let client = create_client();

    let session = Session::load();

    // TO FIX: ensure that the account being logged in is a school/student account, not personal email
    
    let full_url = if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{}{}", session.base_url.parse::<String>().unwrap(), url)
    };

    match client.post(full_url).json(&data).query(&parameters).send().await {
        Ok(resp) => {
            let response = resp.text().await.unwrap();
            println!("{}", response);
            Ok(format!("{}", response))
        },
        Err(e) => Err(format!("HTTP request failed: {e}")),
    }
}