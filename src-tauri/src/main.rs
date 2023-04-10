// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bollard::Docker;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_info,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_info() -> Result<String, String> {
    let info = get_docker_info().await;
    //Ok("Ok".to_string())
    if info.is_err() {
        return Err(String::from("接続できません"));
    }
    Ok(info.unwrap())
}

async fn get_docker_info() -> Result<String, String> {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let info = docker.version().await;
    if info.is_err() {
        return Err(String::from("接続できません"));
    }
    Ok(info.unwrap().version.unwrap())
}
