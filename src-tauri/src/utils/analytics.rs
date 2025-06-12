use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

fn analytics_file() -> PathBuf {
    let mut dir = dirs_next::data_dir().expect("Unable to determine data dir");
    dir.push("DesQTA");
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("Unable to create data dir");
    }
    dir.push("analytics.json");
    dir
}

#[tauri::command]
pub fn save_analytics(data: String) -> Result<(), String> {
    let path = analytics_file();
    fs::write(path, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_analytics() -> Result<String, String> {
    let path = analytics_file();
    fs::read_to_string(path).map_err(|e| e.to_string())
} 