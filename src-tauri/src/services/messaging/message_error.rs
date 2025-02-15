use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("network error")]
    Network(#[from] std::io::Error),
    #[error("serialization error")]
    Serialization(#[from] serde_json::Error),
    #[error("database error")]
    Database(#[from] sea_orm::DbErr),
}
