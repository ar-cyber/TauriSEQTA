use reqwest;
use std::collections::HashMap;


#[tauri::command]
pub async fn get_api_data(url: &str, parameters: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
    let client = reqwest::Client::new();
    match client.get(url)
    .query(&parameters) // Apply parameters
    .send() // Send
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
=======
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
>>>>>>> 1343239473333a877135872173eedaa9e3a06e49
}