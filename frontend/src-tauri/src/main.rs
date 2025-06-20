// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::process::Command;
// use tauri::{command, Builder};

// // No need to import DialogPlugin
// fn main() {
//     Builder::default()
//         .plugin(tauri_plugin_dialog::init())
//         .plugin(tauri_plugin_fs::init())
//         .plugin(tauri_plugin_shell::init())
//         .invoke_handler(tauri::generate_handler![run_parser])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// #[command]
// fn run_parser(file_path: String) -> Result<String, String> {
//     let output = Command::new("python3")
//         .arg("src-tauri/parser/telemetry_parser.py")
//         .arg(&file_path)
//         .output()
//         .map_err(|e| format!("Failed to start parser: {}", e))?;

//     if output.status.success() {
//         Ok(String::from_utf8_lossy(&output.stdout).to_string())
//     } else {
//         Err(String::from_utf8_lossy(&output.stderr).to_string())
//     }
// }




// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// mod commands;
// use std::process::Command;
// use tauri::{command, Builder};

// fn main() {
//     Builder::default()
//         .plugin(tauri_plugin_dialog::init())
//         .plugin(tauri_plugin_fs::init())
//         .plugin(tauri_plugin_shell::init())
//         .invoke_handler(tauri::generate_handler![run_parser])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// #[command]
// fn run_parser(file_path: String) -> Result<String, String> {
//     let output = Command::new("python3")
//         .arg("src-tauri/parser/telemetry_parser.py")
//         .arg(&file_path)
//         .output()
//         .map_err(|e| format!("Failed to start parser: {}", e))?;

//     if output.status.success() {
//         Ok(String::from_utf8_lossy(&output.stdout).to_string())
//     } else {
//         Err(String::from_utf8_lossy(&output.stderr).to_string())
//     }
// }


use app_lib::run;

fn main() {
    run();
}
