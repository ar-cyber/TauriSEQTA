use tauri::{Emitter, Manager};
use time::OffsetDateTime;
use url::Url;
use serde::Deserialize;
use serde_json::json;
use base64::{Engine as _, engine::general_purpose};

use crate::netgrab;
use crate::session;

#[derive(Debug, Deserialize)]
struct SeqtaSSOPayload {
    t: String, // JWT token
    u: String, // Server URL
    n: String, // User number
}

#[derive(Debug, Deserialize)]
struct SeqtaJWT {
    sub: String,    // Subject (user ID)
    exp: i64,       // Expiration timestamp
    t: String,      // Type/role
    scope: String,  // Permission scope
}

#[tauri::command]
pub fn force_reload(app: tauri::AppHandle) {
    app.emit("reload", "hi".to_string()).unwrap();
}

/// True if a saved login session exists.
#[tauri::command]
pub fn check_session_exists() -> bool {
    session::Session::exists()
}

/// Persist the SEQTA `base_url` and `JSESSIONID`.
#[tauri::command]
pub fn save_session(base_url: String, jsessionid: String) -> Result<(), String> {
    session::Session {
        base_url,
        jsessionid,
        additional_cookies: Vec::new(),
    }
    .save()
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn logout() -> bool {
    if let Ok(_) = netgrab::clear_session().await {
        true
    } else {
        false
    }
}

/// Parse and validate a Seqta Learn SSO deeplink
fn parse_deeplink(deeplink: &str) -> Result<SeqtaSSOPayload, String> {
    const DEEPLINK_PREFIX: &str = "seqtalearn://sso/";
    
    println!("[QR_AUTH] Parsing deeplink: {}", deeplink);
    
    if !deeplink.starts_with(DEEPLINK_PREFIX) {
        println!("[QR_AUTH] Error: Invalid deeplink format, expected prefix: {}", DEEPLINK_PREFIX);
        return Err("Invalid Seqta Learn deeplink format".to_string());
    }

    let encoded_payload = &deeplink[DEEPLINK_PREFIX.len()..];
    println!("[QR_AUTH] Encoded payload length: {}", encoded_payload.len());
    
    // First decode the URL encoding
    let url_decoded = urlencoding::decode(encoded_payload)
        .map_err(|e| {
            println!("[QR_AUTH] URL decode error: {}", e);
            format!("Failed to URL decode: {}", e)
        })?;
    
    println!("[QR_AUTH] URL decoded payload length: {}", url_decoded.len());
    
    // Then decode the base64
    let decoded_payload = general_purpose::STANDARD
        .decode(url_decoded.as_bytes())
        .map_err(|e| {
            println!("[QR_AUTH] Base64 decode error: {}", e);
            format!("Failed to base64 decode: {}", e)
        })?;
    
    println!("[QR_AUTH] Base64 decoded payload length: {}", decoded_payload.len());
    
    let payload_str = String::from_utf8(decoded_payload)
        .map_err(|e| {
            println!("[QR_AUTH] UTF8 conversion error: {}", e);
            format!("Failed to convert to string: {}", e)
        })?;
    
    println!("[QR_AUTH] Payload string: {}", payload_str);
    
    let result = serde_json::from_str(&payload_str)
        .map_err(|e| {
            println!("[QR_AUTH] JSON parse error: {}", e);
            format!("Failed to parse JSON: {}", e)
        })?;
    
    println!("[QR_AUTH] Successfully parsed SSO payload: {:?}", result);
    Ok(result)
}

/// Decode and validate a JWT token
fn decode_jwt(token: &str) -> Result<SeqtaJWT, String> {
    println!("[QR_AUTH] Decoding JWT token: {}", format!("{}...", &token[..50.min(token.len())]));
    
    // For now, we'll decode without verification since we don't have the secret
    // In production, you'd want to verify the signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        println!("[QR_AUTH] Error: Invalid JWT format, expected 3 parts, got {}", parts.len());
        return Err("Invalid JWT format".to_string());
    }

    let payload = parts[1];
    println!("[QR_AUTH] JWT payload part length: {}", payload.len());
    
    // Fix base64 padding if needed
    let mut padded_payload = payload.to_string();
    while padded_payload.len() % 4 != 0 {
        padded_payload.push('=');
    }
    println!("[QR_AUTH] Padded payload length: {}", padded_payload.len());
    
    let decoded_payload = general_purpose::STANDARD
        .decode(&padded_payload)
        .map_err(|e| {
            println!("[QR_AUTH] JWT payload decode error: {}", e);
            format!("Failed to decode JWT payload: {}", e)
        })?;
    
    let payload_str = String::from_utf8(decoded_payload)
        .map_err(|e| {
            println!("[QR_AUTH] JWT payload UTF8 error: {}", e);
            format!("Failed to convert JWT payload to string: {}", e)
        })?;
    
    println!("[QR_AUTH] JWT payload string: {}", payload_str);
    
    let result = serde_json::from_str(&payload_str)
        .map_err(|e| {
            println!("[QR_AUTH] JWT payload JSON parse error: {}", e);
            format!("Failed to parse JWT payload: {}", e)
        })?;
    
    println!("[QR_AUTH] Successfully decoded JWT: {:?}", result);
    Ok(result)
}

/// Validate a JWT token
fn validate_token(token: &str) -> Result<bool, String> {
    println!("[QR_AUTH] Validating JWT token");
    let decoded = decode_jwt(token)?;
    let now = chrono::Utc::now().timestamp();
    let is_valid = decoded.exp > now;
    
    println!("[QR_AUTH] Token validation - Current time: {}, Expiry: {}, Valid: {}", now, decoded.exp, is_valid);
    
    if !is_valid {
        println!("[QR_AUTH] Token has expired! Expired {} seconds ago", now - decoded.exp);
    }
    
    Ok(is_valid)
}

/// Perform the QR code authentication flow
async fn perform_qr_auth(sso_payload: SeqtaSSOPayload) -> Result<session::Session, String> {
    println!("[QR_AUTH] ===== QR CODE AUTHENTICATION FLOW START =====");
    println!("[QR_AUTH] 🔐 QR Code Authentication Method: JWT-Based");
    println!("[QR_AUTH] SSO Payload - URL: {}", sso_payload.u);
    println!("[QR_AUTH] SSO Payload - User: {}", sso_payload.n);
    println!("[QR_AUTH] SSO Payload - JWT Token: {}...", &sso_payload.t[..20.min(sso_payload.t.len())]);
    println!("[QR_AUTH] JWT Token Length: {}", sso_payload.t.len());
    println!("[QR_AUTH] JWT Token starts with 'eyJ': {}", sso_payload.t.starts_with("eyJ"));
    
    let client = reqwest::Client::new();
    let base_url = sso_payload.u;
    let token = sso_payload.t;
    
    // Step 1: First login request (empty body)
    println!("[QR_AUTH] Step 1: First login request");
    let first_login_url = format!("{}/seqta/student/login", base_url);
    println!("[QR_AUTH] First login URL: {}", first_login_url);
    
    let first_response = client.post(&first_login_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .header("Referer", &first_login_url)
        .body("{}")
        .send()
        .await
        .map_err(|e| {
            println!("[QR_AUTH] First login request failed: {}", e);
            format!("First login request failed: {}", e)
        })?;

    println!("[QR_AUTH] First login response status: {}", first_response.status());
    if !first_response.status().is_success() {
        let status = first_response.status();
        let error_text = first_response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        println!("[QR_AUTH] First login failed with response: {}", error_text);
        return Err(format!("First login failed with status: {}", status));
    }

    // Step 2: Recovery request
    println!("[QR_AUTH] Step 2: Recovery request");
    let recovery_url = format!("{}/seqta/student/recover", base_url);
    println!("[QR_AUTH] Recovery URL: {}", recovery_url);
    
    let recovery_body = json!({
        "mode": "info",
        "recovery": token
    });
    println!("[QR_AUTH] Recovery body: {}", serde_json::to_string(&recovery_body).unwrap());
    
    let recovery_response = client.post(&recovery_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .header("Referer", &first_login_url)
        .json(&recovery_body)
        .send()
        .await
        .map_err(|e| {
            println!("[QR_AUTH] Recovery request failed: {}", e);
            format!("Recovery request failed: {}", e)
        })?;

    println!("[QR_AUTH] Recovery response status: {}", recovery_response.status());
    if !recovery_response.status().is_success() {
        let status = recovery_response.status();
        let error_text = recovery_response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        println!("[QR_AUTH] Recovery failed with response: {}", error_text);
        return Err(format!("Recovery failed with status: {}", status));
    }

    // Step 3: Second login request with JWT (this is where we get the user data)
    println!("[QR_AUTH] Step 3: Second login request with JWT");
    let second_login_body = json!({
        "jwt": token
    });
    println!("[QR_AUTH] Second login body: {}", serde_json::to_string(&second_login_body).unwrap());
    
    let second_response = client.post(&first_login_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .header("Referer", &first_login_url)
        .json(&second_login_body)
        .send()
        .await
        .map_err(|e| {
            println!("[QR_AUTH] Second login request failed: {}", e);
            format!("Second login request failed: {}", e)
        })?;

    println!("[QR_AUTH] Second login response status: {}", second_response.status());
    if !second_response.status().is_success() {
        let status = second_response.status();
        let error_text = second_response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        println!("[QR_AUTH] Second login failed with response: {}", error_text);
        return Err(format!("Second login failed with status: {}", status));
    }

    // Get the response text to see what we received
    let response_text = second_response.text()
        .await
        .map_err(|e| {
            println!("[QR_AUTH] Failed to read second login response text: {}", e);
            format!("Failed to read second login response: {}", e)
        })?;
    
    println!("[QR_AUTH] Second login response body: {}", response_text);
    println!("[QR_AUTH] Second login response body length: {}", response_text.len());

    // Try to parse the response to see if it contains user data
    if let Ok(response_data) = serde_json::from_str::<serde_json::Value>(&response_text) {
        println!("[QR_AUTH] Successfully parsed second login response: {:?}", response_data);
        
        // Check if we got user data in the payload
        if let Some(payload) = response_data.get("payload") {
            if let Some(user_id) = payload.get("id") {
                println!("[QR_AUTH] Found user ID in response: {}", user_id);
            }
            if let Some(user_name) = payload.get("userName") {
                println!("[QR_AUTH] Found user name in response: {}", user_name);
            }
        }
    } else {
        println!("[QR_AUTH] Second login response is not valid JSON, but continuing");
    }

    println!("[QR_AUTH] ✅ Authentication flow completed successfully");

    // Create session with the JWT token as the session ID
    let session = session::Session {
        base_url,
        jsessionid: token,
        additional_cookies: vec![], // QR auth doesn't use traditional cookies
    };

    println!("[QR_AUTH] 🔐 Session created with JWT-based authentication");
    println!("[QR_AUTH] Session - Base URL: {}", session.base_url);
    println!("[QR_AUTH] Session - JWT Token: {}...", &session.jsessionid[..20.min(session.jsessionid.len())]);
    println!("[QR_AUTH] Session - Additional Cookies: {} (QR auth uses JWT, not cookies)", session.additional_cookies.len());
    println!("[QR_AUTH] ===== QR CODE AUTHENTICATION FLOW END =====");

    Ok(session)
}

/// Open a login window and harvest the cookie once the user signs in.
#[tauri::command]
pub async fn create_login_window(app: tauri::AppHandle, url: String) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};
    use tokio::time::{sleep, Duration};

    println!("[LOGIN] create_login_window called with URL: {}", url);

    // Check if this is a QR code deeplink
    if url.starts_with("seqtalearn://") {
        println!("[LOGIN] Detected QR code deeplink, processing...");
        
        // Parse the deeplink
        let sso_payload = parse_deeplink(&url)?;
        println!("[LOGIN] Successfully parsed SSO payload: {:?}", sso_payload);
        
        // Validate the JWT token
        if !validate_token(&sso_payload.t)? {
            println!("[LOGIN] JWT token validation failed - token expired");
            return Err("JWT token has expired".to_string());
        }
        println!("[LOGIN] JWT token validation successful");
        
        // Perform the QR authentication flow
        let session = perform_qr_auth(sso_payload).await?;
        println!("[LOGIN] QR authentication flow completed successfully");
        
        // Save the session
        match session.save() {
            Ok(_) => println!("[LOGIN] Session saved successfully"),
            Err(e) => {
                println!("[LOGIN] Failed to save session: {}", e);
                return Err(format!("Failed to save session: {}", e));
            }
        }
        
        // Force reload the app
        println!("[LOGIN] Forcing app reload");
        force_reload(app);
        return Ok(());
    }

    let http_url;

    match url.starts_with("https://") {
        true => http_url = url.clone(),
        false => {
            http_url = format!("https://{}", url.clone());
        }
    }

    let parsed_url = match Url::parse(&http_url) {
        Ok(u) => u,
        Err(e) => {
            return Err(format!("Invalid URL: {}", e));
        }
    };

    let full_url: Url = match Url::parse(&format!("{}/#?page=/welcome", parsed_url)) {
        Ok(u) => u,
        Err(e) => {
            return Err(format!("Parsing error: {}", e));
        }
    };

    // Spawn the login window
    WebviewWindowBuilder::new(&app, "seqta_login", WebviewUrl::External(full_url.clone()))
        .title("SEQTA Login")
        .inner_size(900.0, 700.0)
        .build()
        .map_err(|e| format!("Failed to build window: {}", e))?;

    // Clone handles for async block
    let app_handle_clone = app.clone();

    let mut counter = 0; // Creates a counter so that we don't quit authentication upon the first request (which redirects)
                         // Start polling in a background task
    tauri::async_runtime::spawn(async move {
        for _ in 0..1920 {
            // Poll for 1920 seconds max
            // Wait 1 second between polls
            sleep(Duration::from_secs(1)).await;

            // Construct the full URL with the page parameter

            // Try to get cookies from the login window
            if let Some(webview) = app_handle_clone.get_webview_window("seqta_login") {
                if counter > 5 {
                    // Check if the auth has finished through url
                    match webview.url() {
                        Ok(current_url) => {
                            println!("Current URL from webview: {}", current_url);
                            if !(current_url.to_string().contains("#?page=/welcome")) {
                                continue;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error retrieving URL: {}", e);
                        }
                    }

                    match webview.cookies() {
                        Ok(cookies) => {
                            println!("Cookies: {:?}", cookies);
                            for cookie in cookies.clone() {
                                if cookie.name() == "JSESSIONID"
                                    && cookie.domain().unwrap_or("None") == parsed_url.host_str().unwrap_or("None")
                                {
                                    if let Some(expire_time) = cookie.expires_datetime() {
                                        let now = OffsetDateTime::now_utc();
                                        if expire_time > now {
                                            let duration = expire_time - now;
                                            println!(
                                                "Cookie is still valid! Expires in {}",
                                                duration
                                            );

                                            let value = cookie.value().to_string();
                                            let base_url = http_url.clone();

                                            // Convert all cookies to our storage format
                                            let additional_cookies = cookies
                                                .iter()
                                                .filter(|c| c.name() != "JSESSIONID") // Skip JSESSIONID as it's stored separately
                                                .filter(|c| {
                                                    if let Some(cookie_domain) = c.domain() {
                                                        if let Some(host) = parsed_url.host_str() {
                                                            host.ends_with(
                                                                cookie_domain
                                                                    .trim_start_matches('.'),
                                                            )
                                                        } else {
                                                            false
                                                        }
                                                    } else {
                                                        false
                                                    }
                                                }) // only include cookies for the same domain
                                                .map(|c| session::Cookie {
                                                    name: c.name().to_string(),
                                                    value: c.value().to_string(),
                                                    domain: c.domain().map(|s| s.to_string()),
                                                    path: c.path().map(|s| s.to_string()),
                                                })
                                                .collect();

                                            // Save session with all cookies
                                            let session = session::Session {
                                                base_url,
                                                jsessionid: value,
                                                additional_cookies,
                                            };

                                            if let Err(err) = session.save() {
                                                eprintln!("Failed to save session: {}", err);
                                            }

                                            let _ = webview.close();
                                            force_reload(app);
                                            return; // Stop polling once found
                                        } else {
                                            println!("Cookie has expired!");
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error retrieving cookies: {}", err);
                        }
                    }
                }
            }
            counter += 1; // increment the counter at the end of the loop
        }

        eprintln!("JSESSIONID not found within timeout");
    });

    Ok(())
}