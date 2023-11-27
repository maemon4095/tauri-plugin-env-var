// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_handler;
use tauri_plugin_env_var::EnvVarScope;

#[tauri::command]
fn allowed_patterns(scope: tauri::State<'_, EnvVarScope>) -> Vec<(String, String)> {
    scope
        .allowed_patterns()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_env_var::init())
        .invoke_handler(generate_handler![allowed_patterns])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
