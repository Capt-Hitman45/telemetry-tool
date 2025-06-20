// use std::process::Command;
// use tauri::command;

// #[command]
// pub fn run_parser(file_path: String) -> Result<String, String> {
//     let output = Command::new("/home/xdlinx/telemetry-tool/venv/bin/python3")
//         .arg("parser/telemetry_parser.py")
//         .arg(&file_path)
//         .output()
//         .map_err(|e| format!("Failed to start parser: {}", e))?;

//     if output.status.success() {
//         Ok(String::from_utf8_lossy(&output.stdout).to_string())
//     } else {
//         Err(String::from_utf8_lossy(&output.stderr).to_string())
//     }
// }



use std::process::Command;
use tauri::command;

#[command]
pub fn run_parser(file_path: String) -> Result<String, String> {
    Command::new("/home/xdlinx/telemetry-tool/venv/bin/python3")
        .arg("parser/telemetry_parser.py")
        .arg(&file_path)
        .spawn()
        .map_err(|e| format!("Failed to start parser: {}", e))?;

    Ok("Parser started in background.".to_string())
}
