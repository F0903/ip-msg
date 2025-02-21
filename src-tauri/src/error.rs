use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("network error")]
    Network(#[from] std::io::Error),
    #[error("serialization error")]
    Serialization(#[from] serde_json::Error),
    #[error("database error")]
    Database(#[from] sea_orm::DbErr),
    #[error("tauri error")]
    Tauri(#[from] tauri::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
