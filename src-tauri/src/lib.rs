mod backend_error;
mod db;
mod models;
mod utils;

pub use backend_error::BackendError;
use db::LocalDb;
use tauri::Manager;
use utils::AsyncLazyCell;

struct AppState {
    db: AsyncLazyCell<LocalDb, sqlx::Error>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState {
                db: AsyncLazyCell::new(|| Box::pin(LocalDb::new())),
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![db::commands::add_contact])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
