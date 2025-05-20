#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

#[tauri::command]
fn log_punch_out() {
    let now = Utc::now();
    let log = format!("{} - AUTO PUNCH OUT\n", now);
    log_to_file(log);
}

#[tauri::command]
fn log_status(status: String) {
    let now = Utc::now();
    let log = format!("{} - STATUS: {}\n", now, status);
    log_to_file(log);
}

fn log_to_file(entry: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("status_log.txt")
        .expect("Unable to open log file");
    file.write_all(entry.as_bytes()).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![log_status, log_punch_out])
        .setup(|_app| {
            // No window manipulation needed now
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
