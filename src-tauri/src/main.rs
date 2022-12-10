#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
#[tauri::command]
fn check_php() -> bool {
    let output = Command::new("php")
        .arg("-v")
        .status()
        .expect("command php -v failed");
    return output.success();
}

fn get_sniffer_path(handle: tauri::AppHandle) -> std::path::PathBuf {
    return handle
        .path_resolver()
        .resolve_resource("bin/phpcs.phar")
        .expect("could not find phpcs file in resources");
}

#[tauri::command]
fn get_sniffer_languages(handle: tauri::AppHandle) -> String {
    let resource_path = get_sniffer_path(handle);
    let output = Command::new("php")
        .arg(resource_path.as_os_str())
        .arg("-i")
        .output()
        .expect("could not run phpcs");
    return String::from_utf8_lossy(&output.stdout).to_string();
}
#[tauri::command]
fn run_sniffer(handle: tauri::AppHandle, folder_path: String, parser: String) -> String {
    let resource_path = get_sniffer_path(handle);
    let output = Command::new("php")
        .arg(resource_path.as_os_str())
        .arg(format!("--standard={parser}"))
        .arg("--report=json")
        .arg(folder_path)
        .output()
        .expect("could not run phpcs");
    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_php,
            get_sniffer_languages,
            run_sniffer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
