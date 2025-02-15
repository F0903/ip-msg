use dotenv_codegen::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::ConnectOptions;
use services::messaging::MessageService;

mod commands;
mod services;
mod utils;

pub struct AppState {
    db: sea_orm::DatabaseConnection,
    messages: MessageService,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> anyhow::Result<()> {
    log::info!("Starting app...");

    log::info!("Setting up database...");
    let db = sea_orm::Database::connect(
        ConnectOptions::new(dotenv!("DATABASE_URL"))
            .max_connections(10)
            .to_owned(),
    )
    .await?;
    Migrator::up(&db, None).await?;
    log::info!("Done setting up database");

    log::info!("Setting up message service...");
    let messages = MessageService::start(&db).await?;
    log::info!("Done setting up message service");

    log::info!("Setting up tauri...");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(commands::get_handler())
        .manage(AppState { db, messages })
        .run(tauri::generate_context!())?;
    log::info!("Done setting up tauri");

    Ok(())
}
