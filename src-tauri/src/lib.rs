mod commands;
mod error;
mod services;
mod utils;

pub use error::{Error, Result};

use dotenv_codegen::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::ConnectOptions;
use services::{contacts::ContactsService, messaging::MessageService};
use std::sync::Arc;

pub struct AppState {
    contacts: Arc<ContactsService>,
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

    log::info!("Setting up app state...");
    let contacts = Arc::new(ContactsService::new(db.clone()));
    let messages = MessageService::start(db, Arc::clone(&contacts)).await?;
    let state = AppState {
        // We can clone the db because it's an Arc to a Sqlx pool internally.
        contacts,
        messages,
    };

    log::info!("Setting up tauri...");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(commands::get_handler())
        .manage(state)
        .run(tauri::generate_context!())?;

    Ok(())
}
