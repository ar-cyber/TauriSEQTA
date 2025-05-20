#[path = "auth/login.rs"]
mod login;
#[path = "utils/netgrab.rs"]
mod netgrab;
#[path = "utils/settings.rs"]
mod settings;

use tauri_plugin_notification;

/// Boilerplate example command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            netgrab::get_api_data,
            netgrab::post_api_data,
            netgrab::fetch_api_data,
            login::check_session_exists,
            login::save_session,
            login::create_login_window,
            login::logout,
            login::force_reload,
            settings::get_settings,
            settings::save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
