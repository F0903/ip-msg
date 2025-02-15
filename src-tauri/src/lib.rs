use dotenv_codegen::dotenv;
use migration::{Migrator, MigratorTrait};

mod commands;
mod services;
mod utils;

pub struct AppState {
    db: sea_orm::DatabaseConnection,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> anyhow::Result<()> {
    log::info!("Starting app...");

    log::info!("Setting up database...");
    let db = sea_orm::Database::connect(dotenv!("DATABASE_URL")).await?;
    Migrator::up(&db, None).await?;
    log::info!("Done setting up database");

    log::info!("Setting up tauri...");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(commands::get_handler())
        .manage(AppState { db })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    log::info!("Done setting up tauri");

    Ok(())
}
