#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;

// You still don't need tauri::Manager unless you use it elsewhere.
// Remove unused imports to clear warnings.
// use tauri::Manager;

// Import the `init` function from the dialog plugin.
use tauri_plugin_dialog; 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Initialize the dialog plugin using its `init()` function.
        .plugin(tauri_plugin_dialog::init()) 
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![commands::run_parser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}