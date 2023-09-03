// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::HashMap,
    process::{Child, Command},
    sync::Mutex,
};

use crate::config::{IRunningStatusReflection, SSHConfig};
use once_cell::sync::Lazy;
use tauri::Manager;

static SSH_PROCESSES: Lazy<Mutex<HashMap<String, Child>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            app.listen_global("reload", move |_| {
                println!("Reloading...");
                let mut configs = SSHConfig::read_all().unwrap();
                configs.apply_running_status();
                let _ = app_handle.emit_all("update-configs", configs);
            });

            let app_handle = app.app_handle();
            app.listen_global("run", move |event| {
                let name = event
                    .payload()
                    .unwrap()
                    .chars()
                    .skip(1)
                    .take_while(|x| *x != '"')
                    .collect::<String>();
                let child = Command::new("ssh").args([&name]).spawn().unwrap();
                {
                    let mut processes = SSH_PROCESSES.lock().unwrap();
                    processes.insert(name.to_string(), child);
                }
                let _ = app_handle.trigger_global("reload", None);
            });

            let app_handle = app.app_handle();
            app.listen_global("kill", move |event| {
                let name = event
                    .payload()
                    .unwrap()
                    .chars()
                    .skip(1)
                    .take_while(|x| *x != '"')
                    .collect::<String>();
                {
                    let mut processes = SSH_PROCESSES.lock().unwrap();
                    let (_, mut child) = processes.remove_entry(&name).unwrap();
                    child.kill().unwrap();
                }
                let _ = app_handle.trigger_global("reload", None);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod config;
