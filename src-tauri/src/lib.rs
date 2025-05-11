#[path = "utils/netgrab.rs"]
mod netgrab;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String { // Boilerplate default stuff from Tauri, unused
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default() // Build the Tauri app and its exposed methods
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, netgrab::get_api_data, netgrab::post_api_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
