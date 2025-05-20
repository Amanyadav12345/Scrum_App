#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc; // ✅ Explicitly import Utc
use tauri::{SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem, Manager};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                if id.as_str() == "quit" {
                    std::process::exit(0);
                }
            }
            _ => {}
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap(); // Start minimized
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![log_status, log_punch_out])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
#[tauri::command]
fn log_punch_out() {
    let now = Utc::now(); // ✅ This now works
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
        .invoke_handler(tauri::generate_handler![log_punch_out, log_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
