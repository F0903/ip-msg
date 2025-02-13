use dotenv_codegen::dotenv;
use migration::{Migrator, MigratorTrait};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let db = sea_orm::Database::connect(dotenv!("DATABASE_URL")).await?;
    Migrator::up(&db, None).await?;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
