use thiserror::Error;

pub type Result<T> = std::result::Result<T, BackendError>;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("{0}")]
    Database(#[from] crate::db::DbError),
}

impl serde::Serialize for BackendError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::Database(_) => error_message,
        };
        error_kind.serialize(serializer)
    }
}
