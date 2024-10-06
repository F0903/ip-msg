// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use std::net::Ipv4Addr;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

type Result<T> = std::result::Result<T, String>;

#[derive(Deserialize, Debug)]
struct Contact {
    name: String,
    ip: Ipv4Addr,
}

#[tauri::command]
fn add_contact(contact: Contact) -> Result<bool> {
    println!("add_contact called! Contact is: {:?}", contact);
    Ok(true)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_contact])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
