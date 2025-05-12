#[path = "utils/netgrab.rs"]
mod netgrab;
#[path = "auth/login.rs"]
mod login;

/// Boilerplate example command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            netgrab::get_api_data,
            netgrab::post_api_data,
            netgrab::fetch_api_data,
            login::check_session_exists,
            login::save_session,
            login::create_login_window,
            login::logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}