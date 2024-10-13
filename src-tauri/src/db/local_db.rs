use super::error::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub struct LocalDb {
    pool: SqlitePool,
}

impl LocalDb {
    pub async fn new() -> Result<Self> {
        let pool = SqlitePool::connect_lazy_with(
            SqliteConnectOptions::new()
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
                .optimize_on_close(true, None)
                .analysis_limit(250)
                .filename("data.db")
                .create_if_missing(true),
        );
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub const fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
