# 🧠 SCRUM Master Assistant (Tauri Desktop App)

This is a lightweight, cross-platform desktop assistant built with **Tauri + Rust + HTML/JS**. It runs in the background like a digital SCRUM master—detecting inactivity, prompting for status updates, and auto-logging user activity for agile team monitoring.

---

## ✨ Features

- ✅ **Idle Detection:** Prompts user after 10 minutes of inactivity.
- ✅ **Auto Punch-Out:** If no response in 5 minutes, logs a punch-out.
- ✅ **System Tray App:** Runs silently with a quit option.
- ✅ **Log Statuses:** All entries saved in `status_log.txt` locally.
- ✅ **Cross-Platform:** Works on Windows, macOS, and Linux.

---

## 🛠️ Tech Stack

- **Tauri** – Secure, native system APIs with Rust backend
- **Rust** – For low-level system control and logging
- **HTML/JavaScript** – For UI and idle tracking logic
- **chrono** – To timestamp logs

---

## 🚀 Getting Started

### 1. Install Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

```bash
cargo install create-tauri-app
npm install -g @tauri-apps/cli
