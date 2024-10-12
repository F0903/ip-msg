// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod utils;

use db::{ContactsSurface, LocalDb};
use models::Contact;
use serde::Serialize;
use utils::AsyncLazyCell;

type Result<T> = std::result::Result<T, BackendError>;

static DB: AsyncLazyCell<LocalDb, sqlx::Error> = AsyncLazyCell::new(|| Box::pin(LocalDb::new()));

#[derive(Serialize)]
struct BackendError {
    msg: String,
}

impl BackendError {
    pub fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

#[tauri::command]
async fn add_contact(contact: Contact) -> Result<bool> {
    println!("add_contact called! Contact is: {:?}", contact);
    let db = DB.get().await.map_err(|e| BackendError::new(e))?;
    let contacts = ContactsSurface::on(db)
        .await
        .map_err(|e| BackendError::new(e))?;
    contacts
        .write(contact)
        .await
        .map_err(|e| BackendError::new(e))?;
    Ok(true)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![add_contact])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
