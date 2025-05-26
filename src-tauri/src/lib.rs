#[path = "auth/login.rs"]
mod login;
#[path = "utils/netgrab.rs"]
mod netgrab;
#[path = "utils/settings.rs"]
mod settings;

#[cfg(any(target_os = "ios", target_os = "android"))]
#[path = "mobilechanges/login.rs"]
mod mobile_login;

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
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            login::check_session_exists,
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            login::save_session,
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            login::create_login_window,
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            login::logout,
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            login::force_reload,
            #[cfg(any(target_os = "ios", target_os = "android"))]
            mobile_login::check_session_exists,
            #[cfg(any(target_os = "ios", target_os = "android"))]
            mobile_login::save_session,
            #[cfg(any(target_os = "ios", target_os = "android"))]
            mobile_login::create_login_window,
            #[cfg(any(target_os = "ios", target_os = "android"))]
            mobile_login::logout,
            #[cfg(any(target_os = "ios", target_os = "android"))]
            mobile_login::force_reload,
            settings::get_settings,
            settings::save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
