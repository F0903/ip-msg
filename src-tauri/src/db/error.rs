use thiserror::Error;

pub type Result<T> = std::result::Result<T, DbError>;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("value already exists")]
    NotUnique,
    #[error("value was null")]
    Null,
    #[error("check violation")]
    CheckViolation,
    #[error("foreign key violation")]
    ForeignKeyViolation,
    #[error("migration error! {0}")]
    MigrationError(String),
    #[error("unknown db error")]
    Other,
}

impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        match value.into_database_error() {
            Some(e) => match e.kind() {
                sqlx::error::ErrorKind::UniqueViolation => Self::NotUnique,
                sqlx::error::ErrorKind::NotNullViolation => Self::Null,
                sqlx::error::ErrorKind::CheckViolation => Self::CheckViolation,
                sqlx::error::ErrorKind::ForeignKeyViolation => Self::ForeignKeyViolation,
                _ => Self::Other,
            },
            None => Self::Other,
        }
    }
}

impl From<sqlx::migrate::MigrateError> for DbError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::MigrationError(value.to_string())
    }
}
